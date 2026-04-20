param(
  [string]$RepoRoot = "E:\codex\hope-kb"
)

$ErrorActionPreference = "Stop"

$manifestPath = Join-Path $RepoRoot "seed\v0.1\manifest.json"
$importMapPath = Join-Path $RepoRoot "seed\v0.1\import_map.json"

$manifest = Get-Content -Raw -Encoding utf8 $manifestPath | ConvertFrom-Json
$importMap = Get-Content -Raw -Encoding utf8 $importMapPath | ConvertFrom-Json

function Get-CanonicalTextBytes {
  param(
    [string]$Path
  )

  $rawBytes = [System.IO.File]::ReadAllBytes($Path)
  $normalizedBytes = New-Object System.Collections.Generic.List[byte]

  for ($i = 0; $i -lt $rawBytes.Length; $i++) {
    if ($i -lt ($rawBytes.Length - 1) -and $rawBytes[$i] -eq 13 -and $rawBytes[$i + 1] -eq 10) {
      $normalizedBytes.Add(10)
      $i++
      continue
    }

    $normalizedBytes.Add($rawBytes[$i])
  }

  return $normalizedBytes.ToArray()
}

function Load-SeedJson {
  param(
    [string]$RelativePath,
    [string]$Root
  )

  $path = Join-Path $Root ($RelativePath -replace '/', '\')
  if (!(Test-Path $path)) {
    throw "Missing seed file: $RelativePath"
  }

  return Get-Content -Raw -Encoding utf8 $path | ConvertFrom-Json
}

function Get-RecordsFromMapEntry {
  param(
    [pscustomobject]$Entry,
    [string]$Root
  )

  $filePath = Join-Path $Root ($Entry.file -replace '/', '\')
  if (!(Test-Path $filePath)) {
    throw "Missing seed file: $($Entry.file)"
  }

  $json = Get-Content -Raw -Encoding utf8 $filePath | ConvertFrom-Json

  if ($Entry.json_path -eq '$') {
    return @($json)
  }

  $prop = $Entry.json_path
  $records = $json.$prop
  if ($null -eq $records) {
    throw "Missing json_path '$prop' in $($Entry.file)"
  }

  return @($records)
}

function Assert-ValidSceneTaxonomyReferences {
  param(
    [string]$Root
  )

  $sceneTaxonomy = @(Load-SeedJson -RelativePath "seed/v0.1/scene_taxonomy.json" -Root $Root | ForEach-Object { $_ })
  $directorCutSamples = @(Load-SeedJson -RelativePath "seed/v0.1/director_cut_samples.json" -Root $Root | ForEach-Object { $_ })
  $sceneTaxonomyAliases = @(Load-SeedJson -RelativePath "seed/v0.1/scene_taxonomy_aliases.json" -Root $Root | ForEach-Object { $_ })

  $sceneTypes = @{}
  $sceneLookup = @{}
  foreach ($entry in $sceneTaxonomy) {
    $sceneType = [string]$entry.scene_type
    $sceneTypes[$sceneType] = $true
    $sceneLookup[$sceneType] = $sceneType
  }

  foreach ($alias in $sceneTaxonomyAliases) {
    $aliasSceneType = [string]$alias.alias_scene_type
    $canonicalSceneType = [string]$alias.canonical_scene_type

    if (!$sceneTypes.ContainsKey($canonicalSceneType)) {
      throw "Unknown canonical_scene_type '$canonicalSceneType' in scene_taxonomy_aliases: $($alias.machine_id)"
    }

    if ($sceneLookup.ContainsKey($aliasSceneType) -and $sceneLookup[$aliasSceneType] -ne $canonicalSceneType) {
      throw "Conflicting scene alias '$aliasSceneType' in scene_taxonomy_aliases: $($alias.machine_id)"
    }

    $sceneLookup[$aliasSceneType] = $canonicalSceneType
  }

  foreach ($sample in $directorCutSamples) {
    $sceneType = [string]$sample.scene_type
    if (!$sceneLookup.ContainsKey($sceneType)) {
      throw "Unknown scene_type '$sceneType' in director_cut_samples: $($sample.machine_id)"
    }
  }
}

function Assert-ValidRepairMappings {
  param(
    [string]$Root
  )

  $promptTemplates = @(Load-SeedJson -RelativePath "seed/v0.1/prompt_templates.json" -Root $Root | ForEach-Object { $_ })
  $failurePatterns = @(Load-SeedJson -RelativePath "seed/v0.1/failure_pattern_library.json" -Root $Root | ForEach-Object { $_ })

  $promptTemplatesById = @{}
  foreach ($template in $promptTemplates) {
    $promptTemplatesById[[string]$template.machine_id] = $template
  }

  $failureCodes = @{}
  foreach ($failure in $failurePatterns) {
    $failureCodeValue = [string]$failure.failure_code
    if ($failureCodes.ContainsKey($failureCodeValue)) {
      throw "Duplicate failure_code '$failureCodeValue' in failure_pattern_library"
    }
    $failureCodes[$failureCodeValue] = $true
  }

  foreach ($failure in $failurePatterns) {
    $repairTemplateIds = @()
    if ($null -ne $failure.repair_template_ids) {
      $repairTemplateIds = @($failure.repair_template_ids)
    }

    if ($repairTemplateIds.Count -eq 0) {
      throw "Missing repair_template_ids in failure_pattern_library: $($failure.machine_id)"
    }

    $seenTemplateIds = @{}
    foreach ($templateId in $repairTemplateIds) {
      $templateIdValue = [string]$templateId
      if ($seenTemplateIds.ContainsKey($templateIdValue)) {
        throw "Duplicate repair_template_id '$templateIdValue' in failure_pattern_library: $($failure.machine_id)"
      }
      $seenTemplateIds[$templateIdValue] = $true

      if (!$promptTemplatesById.ContainsKey($templateIdValue)) {
        throw "Unknown repair_template_id '$templateIdValue' in failure_pattern_library: $($failure.machine_id)"
      }

      $template = $promptTemplatesById[$templateIdValue]
      if (-not [bool]$template.is_structured_output) {
        throw "Repair template '$templateIdValue' must use structured output"
      }

      $templateStage = [string]$template.stage
      if ($templateStage -ne "repair_pass" -and -not $templateStage.StartsWith("repair_")) {
        throw "Repair template '$templateIdValue' must stay inside repair stages, actual '$templateStage'"
      }

      $templateFailureCodes = @()
      if ($template.PSObject.Properties.Name -contains "repairs_failure_codes" -and $null -ne $template.repairs_failure_codes) {
        $templateFailureCodes = @($template.repairs_failure_codes)
      }

      if ($templateFailureCodes.Count -gt 0 -and !($templateFailureCodes -contains $failure.failure_code)) {
        throw "Repair template '$templateIdValue' does not declare failure_code '$($failure.failure_code)'"
      }
    }
  }

  foreach ($template in $promptTemplates) {
    if (!($template.PSObject.Properties.Name -contains "repairs_failure_codes") -or $null -eq $template.repairs_failure_codes) {
      continue
    }

    if (-not [bool]$template.is_structured_output) {
      throw "Template '$($template.machine_id)' declares repairs_failure_codes but is_structured_output is false"
    }

    $templateStage = [string]$template.stage
    if ($templateStage -ne "repair_pass" -and -not $templateStage.StartsWith("repair_")) {
      throw "Template '$($template.machine_id)' declares repairs_failure_codes but stage is '$templateStage'"
    }

    foreach ($failureCode in @($template.repairs_failure_codes)) {
      $failureCodeValue = [string]$failureCode
      if (!$failureCodes.ContainsKey($failureCodeValue)) {
        throw "Unknown failure_code '$failureCodeValue' in prompt_templates: $($template.machine_id)"
      }
    }
  }
}

function Assert-ValidCommitteeMergeRules {
  param(
    [string]$Root
  )

  $roleDefinitions = @(Load-SeedJson -RelativePath "seed/v0.1/committee_role_definitions.json" -Root $Root | ForEach-Object { $_ })
  $mergeRules = @(Load-SeedJson -RelativePath "seed/v0.1/committee_style_merge_rules.json" -Root $Root | ForEach-Object { $_ })

  $knownRoleCodes = @{}
  foreach ($role in $roleDefinitions) {
    $roleCode = [string]$role.role_code
    $knownRoleCodes[$roleCode] = $true
  }

  $seenRoleCodes = @{}
  $seenPrecedence = @{}
  $precedenceValues = @()

  foreach ($rule in $mergeRules) {
    $roleCode = [string]$rule.role_code
    if (!$knownRoleCodes.ContainsKey($roleCode)) {
      throw "Unknown role_code '$roleCode' in committee_style_merge_rules: $($rule.machine_id)"
    }

    if ($seenRoleCodes.ContainsKey($roleCode)) {
      throw "Duplicate role_code '$roleCode' in committee_style_merge_rules"
    }
    $seenRoleCodes[$roleCode] = $true

    $precedenceOrder = [int]$rule.precedence_order
    if ($precedenceOrder -lt 1) {
      throw "Invalid precedence_order '$precedenceOrder' in committee_style_merge_rules: $($rule.machine_id)"
    }

    if ($seenPrecedence.ContainsKey($precedenceOrder)) {
      throw "Duplicate precedence_order '$precedenceOrder' in committee_style_merge_rules"
    }
    $seenPrecedence[$precedenceOrder] = $true
    $precedenceValues += $precedenceOrder

    $overridableFields = @()
    if ($null -ne $rule.overridable_fields) {
      $overridableFields = @($rule.overridable_fields)
    }

    $nonOverridableFields = @()
    if ($null -ne $rule.non_overridable_fields) {
      $nonOverridableFields = @($rule.non_overridable_fields)
    }

    $fieldCollision = @($overridableFields | Where-Object { $nonOverridableFields -contains $_ } | Select-Object -Unique)
    if ($fieldCollision.Count -gt 0) {
      throw "Field collision in committee_style_merge_rules '$($rule.machine_id)': $($fieldCollision -join ', ')"
    }
  }

  foreach ($roleCode in $knownRoleCodes.Keys) {
    if (!$seenRoleCodes.ContainsKey($roleCode)) {
      throw "Missing committee_style_merge_rule for role_code '$roleCode'"
    }
  }

  $sortedPrecedence = @($precedenceValues | Sort-Object)
  for ($i = 0; $i -lt $sortedPrecedence.Count; $i++) {
    $expected = $i + 1
    if ($sortedPrecedence[$i] -ne $expected) {
      throw "Non-contiguous precedence_order in committee_style_merge_rules: expected $expected, actual $($sortedPrecedence[$i])"
    }
  }
}

function Assert-MinimumDegradedInputCoverage {
  param(
    [string]$Root
  )

  $failurePatterns = @(Load-SeedJson -RelativePath "seed/v0.1/failure_pattern_library.json" -Root $Root | ForEach-Object { $_ })

  $requiredFailureCodes = @(
    "style_drift",
    "character_inconsistency",
    "continuity_break",
    "segment_cross_scene",
    "hard_lock_loss",
    "handoff_gap",
    "chinese_prompt_noise",
    "export_contract_drift"
  )

  $requiredFailureCategories = @(
    "style",
    "consistency",
    "continuity",
    "segmentation",
    "handoff",
    "language",
    "export"
  )

  $seenFailureCodes = @{}
  $seenFailureCategories = @{}
  $seenRepairScopes = @{}

  foreach ($failure in $failurePatterns) {
    $seenFailureCodes[[string]$failure.failure_code] = $true
    $seenFailureCategories[[string]$failure.failure_category] = $true
    $seenRepairScopes[[string]$failure.repair_scope] = $true
  }

  foreach ($failureCode in $requiredFailureCodes) {
    if (!$seenFailureCodes.ContainsKey($failureCode)) {
      throw "Missing degraded-input baseline failure_code '$failureCode' in failure_pattern_library"
    }
  }

  foreach ($failureCategory in $requiredFailureCategories) {
    if (!$seenFailureCategories.ContainsKey($failureCategory)) {
      throw "Missing degraded-input baseline failure_category '$failureCategory' in failure_pattern_library"
    }
  }

  if (!$seenRepairScopes.ContainsKey("render_prompt_only")) {
    throw "Missing negative-boundary repair_scope 'render_prompt_only' in failure_pattern_library"
  }
}

function Assert-ValidDegradedInputExamples {
  param(
    [string]$Root
  )

  $failurePatterns = @(Load-SeedJson -RelativePath "seed/v0.1/failure_pattern_library.json" -Root $Root | ForEach-Object { $_ })
  $promptTemplates = @(Load-SeedJson -RelativePath "seed/v0.1/prompt_templates.json" -Root $Root | ForEach-Object { $_ })
  $degradedInputExamples = @(Load-SeedJson -RelativePath "seed/v0.1/degraded_input_examples.json" -Root $Root | ForEach-Object { $_ })

  $requiredFailureCodes = @(
    "style_drift",
    "character_inconsistency",
    "continuity_break",
    "segment_cross_scene",
    "hard_lock_loss",
    "handoff_gap",
    "chinese_prompt_noise",
    "export_contract_drift"
  )

  $requiredBoundaryFocus = @(
    "style_lock_boundary",
    "character_lock_boundary",
    "continuity_boundary",
    "segmentation_boundary",
    "handoff_boundary",
    "language_boundary",
    "export_contract_boundary"
  )

  $failureByCode = @{}
  foreach ($failure in $failurePatterns) {
    $failureCodeValue = [string]$failure.failure_code
    if ($failureByCode.ContainsKey($failureCodeValue)) {
      throw "Duplicate failure_code '$failureCodeValue' in failure_pattern_library"
    }
    $failureByCode[$failureCodeValue] = $failure
  }

  $promptTemplatesById = @{}
  foreach ($template in $promptTemplates) {
    $promptTemplatesById[[string]$template.machine_id] = $template
  }

  $seenFailureCodes = @{}
  $seenBoundaryFocus = @{}

  foreach ($example in $degradedInputExamples) {
    $failureCode = [string]$example.failure_code
    if (!$failureByCode.ContainsKey($failureCode)) {
      throw "Unknown failure_code '$failureCode' in degraded_input_examples: $($example.machine_id)"
    }

    $failure = $failureByCode[$failureCode]
    $seenFailureCodes[$failureCode] = $true

    foreach ($requiredField in @("example_name", "degradation_mode", "boundary_focus", "minimal_bad_input", "expected_failure_signal", "expected_repair_scope", "pass_condition")) {
      $value = [string]$example.$requiredField
      if ([string]::IsNullOrWhiteSpace($value)) {
        throw "Missing $requiredField in degraded_input_examples: $($example.machine_id)"
      }
    }

    $boundaryFocus = [string]$example.boundary_focus
    $seenBoundaryFocus[$boundaryFocus] = $true

    $expectedRepairScope = [string]$example.expected_repair_scope
    if ($expectedRepairScope -ne [string]$failure.repair_scope) {
      throw "Repair scope mismatch in degraded_input_examples '$($example.machine_id)': expected '$([string]$failure.repair_scope)', actual '$expectedRepairScope'"
    }

    $allowedRepairTemplateIds = @{}
    foreach ($templateId in @($failure.repair_template_ids)) {
      $allowedRepairTemplateIds[[string]$templateId] = $true
    }

    $repairTemplateIds = @($example.repair_template_ids)
    if ($repairTemplateIds.Count -eq 0) {
      throw "Missing repair_template_ids in degraded_input_examples: $($example.machine_id)"
    }

    $seenRepairTemplateIds = @{}
    foreach ($templateId in $repairTemplateIds) {
      $templateIdValue = [string]$templateId
      if ($seenRepairTemplateIds.ContainsKey($templateIdValue)) {
        throw "Duplicate repair_template_id '$templateIdValue' in degraded_input_examples: $($example.machine_id)"
      }
      $seenRepairTemplateIds[$templateIdValue] = $true

      if (!$promptTemplatesById.ContainsKey($templateIdValue)) {
        throw "Unknown repair_template_id '$templateIdValue' in degraded_input_examples: $($example.machine_id)"
      }

      if (!$allowedRepairTemplateIds.ContainsKey($templateIdValue)) {
        throw "Repair template '$templateIdValue' is outside failure_pattern_library mapping for '$failureCode' in degraded_input_examples: $($example.machine_id)"
      }
    }

    $validatorTargets = @($example.validator_targets)
    if ($validatorTargets.Count -eq 0) {
      throw "Missing validator_targets in degraded_input_examples: $($example.machine_id)"
    }

    $allowedValidators = @([string]$failure.validator_hint)
    if ($null -ne $failure.suggested_followup_validator) {
      $allowedValidators += @($failure.suggested_followup_validator | ForEach-Object { [string]$_ })
    }

    $validatorMatch = $false
    foreach ($validatorTarget in $validatorTargets) {
      if ($allowedValidators -contains [string]$validatorTarget) {
        $validatorMatch = $true
        break
      }
    }

    if (-not $validatorMatch) {
      throw "Validator mismatch in degraded_input_examples '$($example.machine_id)': none of the validator_targets map to '$failureCode'"
    }
  }

  foreach ($failureCode in $requiredFailureCodes) {
    if (!$seenFailureCodes.ContainsKey($failureCode)) {
      throw "Missing degraded-input example coverage for failure_code '$failureCode'"
    }
  }

  foreach ($boundaryFocus in $requiredBoundaryFocus) {
    if (!$seenBoundaryFocus.ContainsKey($boundaryFocus)) {
      throw "Missing degraded-input boundary_focus '$boundaryFocus'"
    }
  }
}

function Test-PlaceholderLikeText {
  param(
    [string]$Value
  )

  if ([string]::IsNullOrWhiteSpace($Value)) {
    return $true
  }

  if ($Value.Contains([char]0xFFFD)) {
    return $true
  }

  if ($Value -match '\?{2,}') {
    return $true
  }

  return $false
}

function Assert-ValidClassicCaseExamples {
  param(
    [string]$Root
  )

  $classicCases = @(Load-SeedJson -RelativePath "seed/v0.1/classic_case_examples.json" -Root $Root | ForEach-Object { $_ })
  $committeeTemplates = @(Load-SeedJson -RelativePath "seed/v0.1/committee_templates.json" -Root $Root | ForEach-Object { $_ })
  $directorProfiles = @(Load-SeedJson -RelativePath "seed/v0.1/director_profiles.json" -Root $Root | ForEach-Object { $_ })
  $failurePatterns = @(Load-SeedJson -RelativePath "seed/v0.1/failure_pattern_library.json" -Root $Root | ForEach-Object { $_ })

  if ($classicCases.Count -lt 10) {
    throw "classic_case_examples must include at least 10 records for structural validation"
  }

  $caseProperties = @($classicCases[0].PSObject.Properties | ForEach-Object { $_.Name })
  $committeeProperties = @($committeeTemplates[0].PSObject.Properties | ForEach-Object { $_.Name })
  $directorProperties = @($directorProfiles[0].PSObject.Properties | ForEach-Object { $_.Name })

  $caseNameField = $caseProperties[1]
  $caseTypeField = $caseProperties[2]
  $durationField = $caseProperties[3]
  $committeeField = $caseProperties[4]
  $directorComboField = $caseProperties[5]
  $summaryField = $caseProperties[6]
  $structureField = $caseProperties[7]
  $validationField = $caseProperties[8]

  $committeeNameField = $committeeProperties[1]
  $directorNameField = $directorProperties[1]

  $benchmarkCaseType = [string]$classicCases[0].PSObject.Properties[$caseTypeField].Value
  $classicSegmentCaseType = [string]$classicCases[3].PSObject.Properties[$caseTypeField].Value
  $handoffCaseType = [string]$classicCases[6].PSObject.Properties[$caseTypeField].Value
  $directorSignatureCaseType = [string]$classicCases[9].PSObject.Properties[$caseTypeField].Value
  $failureRepairCaseType = "failure_repair"
  $secondsSuffix = ([string]$classicCases[0].PSObject.Properties[$durationField].Value) -replace '^\d+(?:-\d+)?\s*', ''
  $minutesSuffix = ([string]$classicCases[1].PSObject.Properties[$durationField].Value) -replace '^\d+(?:-\d+)?\s*', ''
  $allowedDurationSuffixes = @($secondsSuffix, $minutesSuffix) | Select-Object -Unique

  $allowedCaseTypes = @(
    $benchmarkCaseType,
    $classicSegmentCaseType,
    $handoffCaseType,
    $directorSignatureCaseType,
    $failureRepairCaseType
  )

  $minimumCaseTypeCounts = @{
    $benchmarkCaseType = 3
    $classicSegmentCaseType = 3
    $handoffCaseType = 5
    $directorSignatureCaseType = 6
    $failureRepairCaseType = 7
  }

  $knownCommittees = @{}
  foreach ($committee in $committeeTemplates) {
    $knownCommittees[[string]$committee.PSObject.Properties[$committeeNameField].Value] = $true
  }

  $knownDirectorNames = @{}
  foreach ($director in $directorProfiles) {
    $knownDirectorNames[[string]$director.PSObject.Properties[$directorNameField].Value] = $true
  }

  $knownFailureCodes = @()
  foreach ($failure in $failurePatterns) {
    $knownFailureCodes += [string]$failure.failure_code
  }

  $caseTypeCounts = @{}
  $requiredFailureRepairCodes = @(
    "style_drift",
    "character_inconsistency",
    "continuity_break",
    "segment_cross_scene",
    "hard_lock_loss",
    "handoff_gap",
    "export_contract_drift",
    "chinese_prompt_noise"
  )
  $failureRepairCoverage = @{}

  foreach ($case in $classicCases) {
    $caseId = [string]$case.machine_id
    $caseType = [string]$case.PSObject.Properties[$caseTypeField].Value

    if ($allowedCaseTypes -notcontains $caseType) {
      throw "Unknown case type '$caseType' in classic_case_examples: $caseId"
    }

    if (!$caseTypeCounts.ContainsKey($caseType)) {
      $caseTypeCounts[$caseType] = 0
    }
    $caseTypeCounts[$caseType]++

    $duration = [string]$case.PSObject.Properties[$durationField].Value
    if ($duration -notmatch '^\d+(?:-\d+)?\s*.+$') {
      throw "Invalid duration '$duration' in classic_case_examples: $caseId"
    }

    $durationSuffix = $duration -replace '^\d+(?:-\d+)?\s*', ''
    if ($allowedDurationSuffixes -notcontains $durationSuffix) {
      throw "Unknown duration suffix '$durationSuffix' in classic_case_examples: $caseId"
    }

    $committeeName = [string]$case.PSObject.Properties[$committeeField].Value
    if (!$knownCommittees.ContainsKey($committeeName)) {
      throw "Unknown committee '$committeeName' in classic_case_examples: $caseId"
    }

    $directorNames = @($case.PSObject.Properties[$directorComboField].Value)
    if ($directorNames.Count -eq 0) {
      throw "Missing director combo in classic_case_examples: $caseId"
    }

    foreach ($directorName in $directorNames) {
      $directorNameValue = [string]$directorName
      if (!$knownDirectorNames.ContainsKey($directorNameValue)) {
        throw "Unknown director combo entry '$directorNameValue' in classic_case_examples: $caseId"
      }
    }

    foreach ($fieldName in @($caseNameField, $summaryField, $structureField, $validationField, "source_notes")) {
      $fieldValue = [string]$case.PSObject.Properties[$fieldName].Value
      if (Test-PlaceholderLikeText -Value $fieldValue) {
        throw "Placeholder-like text detected in classic_case_examples field '$fieldName': $caseId"
      }
    }

    if ($caseType -eq $failureRepairCaseType) {
      $combinedText = @(
        [string]$case.PSObject.Properties[$structureField].Value,
        [string]$case.PSObject.Properties[$validationField].Value,
        [string]$case.source_notes
      ) -join " "

      if ($combinedText -notmatch 'repair|validator') {
        throw "failure_repair case must mention repair or validator flow in classic_case_examples: $caseId"
      }

      $mentionedFailureCodes = @()
      foreach ($failureCode in $knownFailureCodes) {
        if ($combinedText.Contains($failureCode)) {
          $mentionedFailureCodes += $failureCode
          $failureRepairCoverage[$failureCode] = $true
        }
      }

      if ($mentionedFailureCodes.Count -eq 0) {
        throw "failure_repair case must mention at least one failure_code in classic_case_examples: $caseId"
      }
    }
  }

  foreach ($caseType in $minimumCaseTypeCounts.Keys) {
    $actualCount = 0
    if ($caseTypeCounts.ContainsKey($caseType)) {
      $actualCount = [int]$caseTypeCounts[$caseType]
    }

    if ($actualCount -lt [int]$minimumCaseTypeCounts[$caseType]) {
      throw "Insufficient classic_case_examples coverage for case type '$caseType': expected at least $($minimumCaseTypeCounts[$caseType]), actual $actualCount"
    }
  }

  foreach ($failureCode in $requiredFailureRepairCodes) {
    if (!$failureRepairCoverage.ContainsKey($failureCode)) {
      throw "Insufficient failure_repair coverage in classic_case_examples: missing '$failureCode'"
    }
  }
}

function Assert-ValidHopeSupportFlowContracts {
  param(
    [string]$Root
  )

  $failurePatterns = @(Load-SeedJson -RelativePath "seed/v0.1/failure_pattern_library.json" -Root $Root | ForEach-Object { $_ })
  $degradedInputExamples = @(Load-SeedJson -RelativePath "seed/v0.1/degraded_input_examples.json" -Root $Root | ForEach-Object { $_ })
  $classicCases = @(Load-SeedJson -RelativePath "seed/v0.1/classic_case_examples.json" -Root $Root | ForEach-Object { $_ })
  $exportTemplates = @(Load-SeedJson -RelativePath "seed/v0.1/export_templates.json" -Root $Root | ForEach-Object { $_ })
  $promptTemplates = @(Load-SeedJson -RelativePath "seed/v0.1/prompt_templates.json" -Root $Root | ForEach-Object { $_ })

  $failureByCode = @{}
  foreach ($failure in $failurePatterns) {
    $failureByCode[[string]$failure.failure_code] = $failure
  }

  $promptTemplatesById = @{}
  foreach ($template in $promptTemplates) {
    $promptTemplatesById[[string]$template.machine_id] = $template
  }

  $degradedByFailureCode = @{}
  foreach ($example in $degradedInputExamples) {
    $failureCode = [string]$example.failure_code
    if (!$degradedByFailureCode.ContainsKey($failureCode)) {
      $degradedByFailureCode[$failureCode] = @()
    }
    $degradedByFailureCode[$failureCode] += ,$example
  }

  $caseProperties = @($classicCases[0].PSObject.Properties | ForEach-Object { $_.Name })
  $caseTypeField = $caseProperties[2]
  $structureField = $caseProperties[7]
  $validationField = $caseProperties[8]
  $failureRepairCases = @($classicCases | Where-Object { [string]$_.PSObject.Properties[$caseTypeField].Value -eq "failure_repair" })

  $exportTemplatesBySheet = @{}
  foreach ($template in $exportTemplates) {
    $exportTemplatesBySheet[[string]$template.sheet_name] = $template
  }

  $requiredSheetContracts = @{
    "HandoffZones" = @(
      "project_handoff_zones.render_segment_id",
      "project_handoff_zones.buffer_cut_ids"
    )
    "PromptPackage" = @(
      "prompt_packages.render_prompt",
      "prompt_packages.render_segment_id"
    )
    "Validation" = @(
      "validation_report.validator_name",
      "validation_report.is_blocking"
    )
  }

  $requiredSheetCoreFields = @{
    "HandoffZones" = @("buffer_cut_ids", "continuity_notes")
    "PromptPackage" = @("layout_prompt", "render_prompt")
    "Validation" = @("validator_name", "is_blocking")
  }

  foreach ($sheetName in $requiredSheetContracts.Keys) {
    if (!$exportTemplatesBySheet.ContainsKey($sheetName)) {
      throw "Missing support-facing export sheet '$sheetName' in export_templates"
    }

    $templateProperties = @($exportTemplatesBySheet[$sheetName].PSObject.Properties | Where-Object { $_.MemberType -eq "NoteProperty" })
    $coreFields = @($templateProperties[4].Value | ForEach-Object { [string]$_ })
    foreach ($requiredField in $requiredSheetCoreFields[$sheetName]) {
      if ($coreFields -notcontains [string]$requiredField) {
        throw "Support-facing export sheet '$sheetName' is missing core field '$requiredField'"
      }
    }

    $columnSources = @()
    $columnDefinitions = @($templateProperties[5].Value | ForEach-Object { $_ })
    foreach ($column in $columnDefinitions) {
      $columnProperties = @($column.PSObject.Properties | Where-Object { $_.MemberType -eq "NoteProperty" })
      if ($columnProperties.Count -lt 2) {
        throw "Support-facing export sheet '$sheetName' has malformed column definitions"
      }
      $columnSources += [string]$columnProperties[1].Value
    }

    foreach ($requiredSource in $requiredSheetContracts[$sheetName]) {
      if ($columnSources -notcontains $requiredSource) {
        throw "Support-facing export sheet '$sheetName' is missing source '$requiredSource'"
      }
    }
  }

  $supportContracts = @(
    @{
      failure_code = "handoff_gap"
      boundary_focus = "handoff_boundary"
      repair_scope = "handoff_zone_and_buffer_cuts"
      required_layers = @("project_handoff_zones", "storyboard_cuts")
      required_validators = @("Handoff Coverage", "Continuity Validator")
      case_keywords = @("handoff", "transition bridge", "buffer cut")
      required_templates = @(
        @{
          machine_id = "prompt_18"
          stage = "repair_handoff_boundaries"
          expected_output_schema = "HandoffZoneRepair"
          required_inputs = @("failed_handoffs", "adjacent_cuts", "committee_handoff_rules", "current_handoff_zones")
        }
      )
    },
    @{
      failure_code = "export_contract_drift"
      boundary_focus = "export_contract_boundary"
      repair_scope = "export_mapping"
      required_layers = @("export_templates", "validators")
      required_validators = @("Export contract check")
      case_keywords = @("PromptPackage", "sheet", "export")
      required_templates = @(
        @{
          machine_id = "prompt_17"
          stage = "repair_export_contract"
          expected_output_schema = "ExportContractRepair"
          required_inputs = @("validation_findings", "export_templates", "shared_contract_rows", "current_exports")
        }
      )
    },
    @{
      failure_code = "chinese_prompt_noise"
      boundary_focus = "language_boundary"
      repair_scope = "prompt_rendering_layer"
      required_layers = @("prompt_packages", "export_templates")
      required_validators = @("Prompt quality review / Export sanity check")
      case_keywords = @("中文", "prompt", "token")
      required_templates = @(
        @{
          machine_id = "prompt_09"
          stage = "repair_pass"
          expected_output_schema = "SameAsTarget"
          required_inputs = @("validator_failures", "current_payload", "target_schema")
        }
      )
    }
  )

  foreach ($contract in $supportContracts) {
    $failureCode = [string]$contract.failure_code
    if (!$failureByCode.ContainsKey($failureCode)) {
      throw "Missing support-facing failure contract '$failureCode' in failure_pattern_library"
    }

    $failure = $failureByCode[$failureCode]
    $affectedLayers = @($failure.affected_layers | ForEach-Object { [string]$_ })
    $repairTemplateIds = @($failure.repair_template_ids | ForEach-Object { [string]$_ })
    foreach ($requiredLayer in @($contract.required_layers)) {
      if ($affectedLayers -notcontains [string]$requiredLayer) {
        throw "Support-facing failure '$failureCode' is missing affected_layer '$requiredLayer'"
      }
    }

    $allowedValidators = @([string]$failure.validator_hint)
    if ($null -ne $failure.suggested_followup_validator) {
      $allowedValidators += @($failure.suggested_followup_validator | ForEach-Object { [string]$_ })
    }

    foreach ($requiredValidator in @($contract.required_validators)) {
      if ($allowedValidators -notcontains [string]$requiredValidator) {
        throw "Support-facing failure '$failureCode' is missing validator '$requiredValidator'"
      }
    }

    foreach ($templateContract in @($contract.required_templates)) {
      $templateId = [string]$templateContract.machine_id
      if ($repairTemplateIds -notcontains $templateId) {
        throw "Support-facing failure '$failureCode' is missing required repair template '$templateId'"
      }

      if (!$promptTemplatesById.ContainsKey($templateId)) {
        throw "Unknown support-facing repair template '$templateId'"
      }

      $template = $promptTemplatesById[$templateId]
      if ([string]$template.stage -ne [string]$templateContract.stage) {
        throw "Support-facing repair template '$templateId' has unexpected stage '$([string]$template.stage)'"
      }

      if ([string]$template.expected_output_schema -ne [string]$templateContract.expected_output_schema) {
        throw "Support-facing repair template '$templateId' has unexpected output schema '$([string]$template.expected_output_schema)'"
      }

      $templateInputs = @($template.required_inputs | ForEach-Object { [string]$_ })
      foreach ($requiredInput in @($templateContract.required_inputs)) {
        if ($templateInputs -notcontains [string]$requiredInput) {
          throw "Support-facing repair template '$templateId' is missing required input '$requiredInput'"
        }
      }
    }

    if (!$degradedByFailureCode.ContainsKey($failureCode)) {
      throw "Missing degraded_input_examples coverage for support-facing failure '$failureCode'"
    }

    $matchedDegradedExample = $false
    foreach ($example in @($degradedByFailureCode[$failureCode])) {
      if ([string]$example.boundary_focus -ne [string]$contract.boundary_focus) {
        continue
      }

      if ([string]$example.expected_repair_scope -ne [string]$contract.repair_scope) {
        continue
      }

      $validatorTargets = @($example.validator_targets | ForEach-Object { [string]$_ })
      $missingValidator = $false
      foreach ($requiredValidator in @($contract.required_validators)) {
        if ($validatorTargets -notcontains [string]$requiredValidator) {
          $missingValidator = $true
          break
        }
      }

      if (!$missingValidator) {
        $matchedDegradedExample = $true
        break
      }
    }

    if (-not $matchedDegradedExample) {
      throw "Support-facing failure '$failureCode' is missing a degraded_input_example with the expected boundary / repair / validator contract"
    }

    $matchedClassicCase = $false
    foreach ($case in $failureRepairCases) {
      $combinedText = @(
        [string]$case.PSObject.Properties[$structureField].Value,
        [string]$case.PSObject.Properties[$validationField].Value,
        [string]$case.source_notes
      ) -join " "

      if (!$combinedText.Contains($failureCode)) {
        continue
      }

      $keywordMatch = $false
      foreach ($keyword in @($contract.case_keywords)) {
        if ($combinedText -match [Regex]::Escape([string]$keyword)) {
          $keywordMatch = $true
          break
        }
      }

      if ($keywordMatch) {
        $matchedClassicCase = $true
        break
      }
    }

    if (-not $matchedClassicCase) {
      throw "Support-facing failure '$failureCode' is missing a semantically aligned failure_repair case in classic_case_examples"
    }
  }
}

function Assert-ValidCommitteeTopology {
  param(
    [string]$Root
  )

  $roleDefinitions = @(Load-SeedJson -RelativePath "seed/v0.1/committee_role_definitions.json" -Root $Root | ForEach-Object { $_ })
  $directorProfiles = @(Load-SeedJson -RelativePath "seed/v0.1/director_profiles.json" -Root $Root | ForEach-Object { $_ })
  $committeeTemplates = @(Load-SeedJson -RelativePath "seed/v0.1/committee_templates.json" -Root $Root | ForEach-Object { $_ })
  $committeeHandoffRules = @(Load-SeedJson -RelativePath "seed/v0.1/committee_handoff_rules.json" -Root $Root | ForEach-Object { $_ })
  $sceneTaxonomy = @(Load-SeedJson -RelativePath "seed/v0.1/scene_taxonomy.json" -Root $Root | ForEach-Object { $_ })
  $directorCutSamples = @(Load-SeedJson -RelativePath "seed/v0.1/director_cut_samples.json" -Root $Root | ForEach-Object { $_ })
  $directorSceneAffinity = @(Load-SeedJson -RelativePath "seed/v0.1/director_scene_affinity.json" -Root $Root | ForEach-Object { $_ })

  $knownRoleCodes = @{}
  foreach ($role in $roleDefinitions) {
    $knownRoleCodes[[string]$role.role_code] = $true
  }

  $knownDirectorIds = @{}
  foreach ($director in $directorProfiles) {
    $knownDirectorIds[[string]$director.machine_id] = $true
  }

  $knownHandoffPairs = @{}
  foreach ($rule in $committeeHandoffRules) {
    $fromRole = [string]$rule.from_role
    $toRole = [string]$rule.to_role

    if (!$knownRoleCodes.ContainsKey($fromRole)) {
      throw "Unknown from_role '$fromRole' in committee_handoff_rules: $($rule.machine_id)"
    }

    if (!$knownRoleCodes.ContainsKey($toRole)) {
      throw "Unknown to_role '$toRole' in committee_handoff_rules: $($rule.machine_id)"
    }

    $pairKey = "$fromRole->$toRole"
    $knownHandoffPairs[$pairKey] = $true

    foreach ($directorPair in @($rule.applicable_director_pairs)) {
      $parts = @(([string]$directorPair) -split '->')
      if ($parts.Count -ne 2) {
        throw "Invalid applicable_director_pairs entry '$directorPair' in committee_handoff_rules: $($rule.machine_id)"
      }

      foreach ($directorId in $parts) {
        if (!$knownDirectorIds.ContainsKey($directorId)) {
          throw "Unknown director '$directorId' in committee_handoff_rules: $($rule.machine_id)"
        }
      }
    }
  }

  foreach ($template in $committeeTemplates) {
    $chiefDirectorId = [string]$template.chief_director_id
    if (!$knownDirectorIds.ContainsKey($chiefDirectorId)) {
      throw "Unknown chief_director_id '$chiefDirectorId' in committee_templates: $($template.machine_id)"
    }

    $defaultRoles = @{}
    foreach ($prop in $template.default_roles.PSObject.Properties) {
      $roleCode = [string]$prop.Name
      $directorId = [string]$prop.Value

      if (!$knownRoleCodes.ContainsKey($roleCode)) {
        throw "Unknown role_code '$roleCode' in committee_templates: $($template.machine_id)"
      }

      if (!$knownDirectorIds.ContainsKey($directorId)) {
        throw "Unknown director '$directorId' in committee_templates: $($template.machine_id)"
      }

      $defaultRoles[$roleCode] = $directorId
    }

    if (!$defaultRoles.ContainsKey("chief")) {
      throw "Missing chief default role in committee_templates: $($template.machine_id)"
    }

    if ($defaultRoles["chief"] -ne $chiefDirectorId) {
      throw "committee_templates chief mismatch in $($template.machine_id): chief_director_id '$chiefDirectorId' vs default_roles.chief '$($defaultRoles["chief"])'"
    }
  }

  foreach ($scene in $sceneTaxonomy) {
    foreach ($roleCode in @($scene.typical_committee_roles)) {
      $roleCodeValue = [string]$roleCode
      if (!$knownRoleCodes.ContainsKey($roleCodeValue)) {
        throw "Unknown typical_committee_role '$roleCodeValue' in scene_taxonomy: $($scene.machine_id)"
      }
    }

    foreach ($handoffPair in @($scene.default_handoff_out)) {
      $handoffPairValue = [string]$handoffPair
      if (!$knownHandoffPairs.ContainsKey($handoffPairValue)) {
        throw "Unknown default_handoff_out '$handoffPairValue' in scene_taxonomy: $($scene.machine_id)"
      }
    }
  }

  foreach ($sample in $directorCutSamples) {
    $roleCode = [string]$sample.committee_role_fit
    if (!$knownRoleCodes.ContainsKey($roleCode)) {
      throw "Unknown committee_role_fit '$roleCode' in director_cut_samples: $($sample.machine_id)"
    }
  }

  foreach ($affinity in $directorSceneAffinity) {
    $directorId = [string]$affinity.director_machine_id
    if (!$knownDirectorIds.ContainsKey($directorId)) {
      throw "Unknown director_machine_id '$directorId' in director_scene_affinity: $($affinity.machine_id)"
    }

    foreach ($roleCode in @($affinity.primary_role_bias, $affinity.secondary_role_bias)) {
      $roleCodeValue = [string]$roleCode
      if (!$knownRoleCodes.ContainsKey($roleCodeValue)) {
        throw "Unknown role bias '$roleCodeValue' in director_scene_affinity: $($affinity.machine_id)"
      }
    }

    foreach ($directorIdValue in @($affinity.preferred_handoff_targets)) {
      $handoffDirectorId = [string]$directorIdValue
      if (!$knownDirectorIds.ContainsKey($handoffDirectorId)) {
        throw "Unknown preferred_handoff_target '$handoffDirectorId' in director_scene_affinity: $($affinity.machine_id)"
      }
    }
  }
}

$seenMachineIds = @{}
$results = @()

foreach ($entry in $importMap) {
  $records = Get-RecordsFromMapEntry -Entry $entry -Root $RepoRoot
  $actualCount = $records.Count
  $expectedCount = $manifest.record_counts.($entry.record_count_key)

  if ($null -eq $expectedCount) {
    throw "Missing record_counts key '$($entry.record_count_key)' in manifest"
  }

  if ($actualCount -ne [int]$expectedCount) {
    throw "Count mismatch for $($entry.file) [$($entry.json_path)]: expected $expectedCount, actual $actualCount"
  }

  foreach ($record in $records) {
    $pk = $record.($entry.primary_key_field)
    if ([string]::IsNullOrWhiteSpace($pk)) {
      throw "Missing primary key '$($entry.primary_key_field)' in $($entry.file)"
    }
    if ($seenMachineIds.ContainsKey($pk)) {
      throw "Duplicate machine_id detected: $pk"
    }
    $seenMachineIds[$pk] = $true
  }

  $results += [pscustomobject]@{
    file = $entry.file
    table = $entry.table
    json_path = $entry.json_path
    count = $actualCount
  }
}

Assert-ValidSceneTaxonomyReferences -Root $RepoRoot
Assert-ValidRepairMappings -Root $RepoRoot
Assert-ValidCommitteeMergeRules -Root $RepoRoot
Assert-MinimumDegradedInputCoverage -Root $RepoRoot
Assert-ValidDegradedInputExamples -Root $RepoRoot
Assert-ValidClassicCaseExamples -Root $RepoRoot
Assert-ValidHopeSupportFlowContracts -Root $RepoRoot
Assert-ValidCommitteeTopology -Root $RepoRoot

$bundleHash = $manifest.content_hash -replace '^bundle-sha256:', ''
$bytes = New-Object System.Collections.Generic.List[byte]
foreach ($rel in $manifest.bundle_order) {
  $path = Join-Path $RepoRoot ($rel -replace '/', '\')
  if (!(Test-Path $path)) {
    throw "Missing bundle_order file: $rel"
  }
  # Hash canonical LF bytes so Windows autocrlf does not invalidate the bundle.
  $canonicalBytes = [byte[]](Get-CanonicalTextBytes -Path $path)
  $bytes.AddRange($canonicalBytes)
}
$sha = [System.Security.Cryptography.SHA256]::Create()
$actualHash = [System.BitConverter]::ToString($sha.ComputeHash($bytes.ToArray())).Replace('-', '').ToLower()
if ($actualHash -ne $bundleHash) {
  throw "Bundle hash mismatch: expected $bundleHash, actual $actualHash"
}

$results | Format-Table -AutoSize
Write-Output ""
Write-Output "Seed bundle validation passed."
