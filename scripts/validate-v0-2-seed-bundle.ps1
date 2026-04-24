param(
  [string]$RepoRoot = "E:\codex\hope-kb"
)

$ErrorActionPreference = "Stop"

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

function Load-Json {
  param(
    [string]$Path
  )

  return Get-Content -Raw -Encoding utf8 $Path | ConvertFrom-Json
}

$manifestPath = Join-Path $RepoRoot "seed\v0.2\manifest.json"
$importMapPath = Join-Path $RepoRoot "seed\v0.2\import_map.json"
$libraryPath = Join-Path $RepoRoot "seed\v0.2\golden_sample_library.json"
$coveragePath = Join-Path $RepoRoot "seed\v0.2\golden_sample_field_coverage_rules.json"
$failurePath = Join-Path $RepoRoot "seed\v0.2\golden_sample_failure_mapping.json"
$repairPath = Join-Path $RepoRoot "seed\v0.2\golden_sample_repair_mapping.json"
$sourceRegisterPath = Join-Path $RepoRoot "seed\v0.2\source_register.json"

$manifest = Load-Json -Path $manifestPath
$importMap = Load-Json -Path $importMapPath
$library = Load-Json -Path $libraryPath
$coverage = Load-Json -Path $coveragePath
$failure = Load-Json -Path $failurePath
$repair = Load-Json -Path $repairPath
$sourceRegister = Load-Json -Path $sourceRegisterPath

$results = @()

foreach ($entry in $importMap) {
  $path = Join-Path $RepoRoot (([string]$entry.file) -replace '/', '\')
  $json = Load-Json -Path $path
  $records = @($json.($entry.json_path))
  $expected = [int]$manifest.record_counts.($entry.record_count_key)
  $actual = $records.Count

  if ($actual -ne $expected) {
    throw "Count mismatch for $($entry.file) [$($entry.json_path)]: expected $expected actual $actual"
  }

  $seen = @{}
  foreach ($record in $records) {
    $pk = [string]$record.($entry.primary_key_field)
    if ([string]::IsNullOrWhiteSpace($pk)) {
      throw "Missing primary key '$($entry.primary_key_field)' in $($entry.file)"
    }
    if ($seen.ContainsKey($pk)) {
      throw "Duplicate primary key '$pk' in $($entry.file)"
    }
    $seen[$pk] = $true
  }

  $results += [pscustomobject]@{
    file = $entry.file
    count = $actual
  }
}

$libraryRecords = @($library.records)
$coverageRecords = @($coverage.records)
$failureRecords = @($failure.records)
$repairRecords = @($repair.records)
$sourceRecords = @($sourceRegister.sources)
$provenanceRecords = @($sourceRegister.provenance_entries)

if ($library.record_count -ne $libraryRecords.Count) {
  throw "golden_sample_library record_count mismatch"
}

if ($failure.record_count -ne $failureRecords.Count) {
  throw "golden_sample_failure_mapping record_count mismatch"
}

if ($repair.record_count -ne $repairRecords.Count) {
  throw "golden_sample_repair_mapping record_count mismatch"
}

if ($coverage.record_count -ne $coverageRecords.Count) {
  throw "golden_sample_field_coverage_rules record_count mismatch"
}

if ($coverageRecords.Count -ne 5) {
  throw "Expected 5 coverage rules, actual $($coverageRecords.Count)"
}

$libraryBySampleId = @{}
foreach ($record in $libraryRecords) {
  $sampleId = [string]$record.sample_id
  if ($libraryBySampleId.ContainsKey($sampleId)) {
    throw "Duplicate sample_id in golden_sample_library: $sampleId"
  }
  $libraryBySampleId[$sampleId] = $record
}

$failureById = @{}
foreach ($record in $failureRecords) {
  $mappingId = [string]$record.mapping_id
  $sampleId = [string]$record.sample_id
  if (!$libraryBySampleId.ContainsKey($sampleId)) {
    throw "Failure mapping '$mappingId' references missing sample_id '$sampleId'"
  }
  $failureById[$mappingId] = $record
}

foreach ($record in $repairRecords) {
  $mappingId = [string]$record.mapping_id
  $sampleId = [string]$record.sample_id
  $failureMappingId = [string]$record.linked_failure_mapping_id

  if (!$libraryBySampleId.ContainsKey($sampleId)) {
    throw "Repair mapping '$mappingId' references missing sample_id '$sampleId'"
  }

  if (!$failureById.ContainsKey($failureMappingId)) {
    throw "Repair mapping '$mappingId' references missing failure mapping '$failureMappingId'"
  }
}

$allSampleIds = @($libraryRecords | ForEach-Object { [string]$_.sample_id })
$expectedSampleIdSet = @{}
foreach ($sampleId in $allSampleIds) {
  $expectedSampleIdSet[$sampleId] = $true
}

$expectedSequenceCounts = @{
  "CNWARSEQ01" = 4
  "CNWARSEQ02" = 4
  "CNWARSEQ03" = 4
  "CNWARSEQ04" = 4
  "SLGSEQ01" = 4
  "SLGSEQ02" = 4
  "SLGSEQ03" = 4
  "SLGSEQ04" = 4
}

foreach ($rule in $coverageRecords) {
  if ([int]$rule.row_count -ne 152) {
    throw "Coverage rule '$($rule.rule_id)' row_count must be 152"
  }

  $sampleIds = @($rule.source_sample_ids | ForEach-Object { [string]$_ })
  if ($sampleIds.Count -ne 152) {
    throw "Coverage rule '$($rule.rule_id)' must carry 152 source_sample_ids"
  }

  $sampleSet = @{}
  foreach ($sampleId in $sampleIds) {
    if ($sampleSet.ContainsKey($sampleId)) {
      throw "Coverage rule '$($rule.rule_id)' has duplicate source_sample_id '$sampleId'"
    }
    $sampleSet[$sampleId] = $true
  }

  foreach ($sampleId in $expectedSampleIdSet.Keys) {
    if (!$sampleSet.ContainsKey($sampleId)) {
      throw "Coverage rule '$($rule.rule_id)' is missing source_sample_id '$sampleId'"
    }
  }

  $summary = $rule.coverage_summary
  if ([int]$summary.library_status.official -ne 108 -or [int]$summary.library_status.reserve -ne 44) {
    throw "Coverage rule '$($rule.rule_id)' library_status summary mismatch"
  }
  if ([int]$summary.sample_type.single_shot -ne 72 -or [int]$summary.sample_type.sequence_shot -ne 80) {
    throw "Coverage rule '$($rule.rule_id)' sample_type summary mismatch"
  }
  if ([int]$summary.usable_for_fewshot.Yes -ne 97 -or [int]$summary.usable_for_fewshot.No -ne 55) {
    throw "Coverage rule '$($rule.rule_id)' usable_for_fewshot summary mismatch"
  }
  $qualityGradeTotal = ($summary.quality_grade.PSObject.Properties | Measure-Object -Property Value -Sum).Sum
  if ([int]$qualityGradeTotal -ne 152) {
    throw "Coverage rule '$($rule.rule_id)' quality_grade summary total mismatch"
  }
  foreach ($sequenceId in $expectedSequenceCounts.Keys) {
    if ([int]$summary.sequence_groups.$sequenceId -ne [int]$expectedSequenceCounts[$sequenceId]) {
      throw "Coverage rule '$($rule.rule_id)' sequence_groups mismatch for $sequenceId"
    }
  }
  if ([int]$summary.surface_completeness.($rule.core) -ne 152) {
    throw "Coverage rule '$($rule.rule_id)' surface completeness mismatch for $($rule.core)"
  }
}

$newRows = @($libraryRecords | Where-Object { [string]$_.sample_id -like 'GS120-CAND-*' })
if ($newRows.Count -ne 32) {
  throw "Expected 32 new GS120-CAND rows, actual $($newRows.Count)"
}

foreach ($row in $newRows) {
  if ([string]$row.classification.library_status -ne "reserve") {
    throw "New row '$($row.sample_id)' is not reserve"
  }
  if ([bool]$row.classification.usable_for_fewshot) {
    throw "New row '$($row.sample_id)' classification.usable_for_fewshot must be false"
  }
  if ([bool]$row.fewshot.eligible) {
    throw "New row '$($row.sample_id)' fewshot.eligible must be false"
  }
  if ([string]$row.fewshot.source_value -ne "No") {
    throw "New row '$($row.sample_id)' fewshot.source_value must be No"
  }
  if ([string]$row.fewshot.retrieval_status -ne "reserve_holdout_not_for_positive_fewshot") {
    throw "New row '$($row.sample_id)' fewshot.retrieval_status mismatch"
  }
  if ([string]$row.source_fields.library_status -ne "reserve" -or [string]$row.source_fields.usable_for_fewshot -ne "No") {
    throw "New row '$($row.sample_id)' source_fields reserve/No mismatch"
  }
  foreach ($field in @("technical_profile", "scene_performance_core", "camera_directing_core", "audio_directing_core", "continuity_negative_core")) {
    if ([string]::IsNullOrWhiteSpace([string]$row.source_fields.$field)) {
      throw "New row '$($row.sample_id)' missing source field '$field'"
    }
  }
}

if ($sourceRecords.Count -ne 14) {
  throw "source_register sources count mismatch"
}

if ($provenanceRecords.Count -ne 4) {
  throw "source_register provenance_entries count mismatch"
}

$bundleHash = $manifest.content_hash -replace '^bundle-sha256:', ''
$bytes = New-Object System.Collections.Generic.List[byte]
foreach ($rel in $manifest.bundle_order) {
  $path = Join-Path $RepoRoot ($rel -replace '/', '\')
  if (!(Test-Path $path)) {
    throw "Missing bundle_order file: $rel"
  }
  $canonicalBytes = [byte[]](Get-CanonicalTextBytes -Path $path)
  $bytes.AddRange($canonicalBytes)
}
$sha = [System.Security.Cryptography.SHA256]::Create()
$actualHash = [System.BitConverter]::ToString($sha.ComputeHash($bytes.ToArray())).Replace('-', '').ToLower()
if ($actualHash -ne $bundleHash) {
  throw "Bundle hash mismatch: expected $bundleHash actual $actualHash"
}

$results | Format-Table -AutoSize
Write-Output ""
Write-Output "v0.2 seed bundle validation passed."
