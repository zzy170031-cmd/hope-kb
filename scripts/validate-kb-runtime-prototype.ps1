param(
    [string]$RepoRoot = (Resolve-Path "$PSScriptRoot\..").Path,
    [string]$SnapshotPath = "knowledge\runtime_snapshots\latest.candidate.json",
    [string]$MappingPath = "knowledge\mappings\wiki-to-runtime-mapping.v0.2.json",
    [string]$PwaAdapterPath = "samples\pwa-kb-adapter-output.sample.json"
)

$ErrorActionPreference = "Stop"

$repo = (Resolve-Path $RepoRoot).Path
$snapshotFullPath = Join-Path $repo $SnapshotPath
$mappingFullPath = Join-Path $repo $MappingPath
$pwaAdapterFullPath = Join-Path $repo $PwaAdapterPath

if (-not (Test-Path $snapshotFullPath)) {
    Write-Error "Snapshot not found: $SnapshotPath"
}
if (-not (Test-Path $mappingFullPath)) {
    Write-Error "Mapping not found: $MappingPath"
}
if (-not (Test-Path $pwaAdapterFullPath)) {
    Write-Error "PWA adapter sample not found: $PwaAdapterPath"
}

$snapshot = Get-Content -Raw -LiteralPath $snapshotFullPath | ConvertFrom-Json
$mapping = Get-Content -Raw -LiteralPath $mappingFullPath | ConvertFrom-Json
$pwaAdapter = Get-Content -Raw -LiteralPath $pwaAdapterFullPath | ConvertFrom-Json

$errors = New-Object System.Collections.Generic.List[string]

function Add-Error([string]$Message) {
    $script:errors.Add($Message) | Out-Null
}

$requiredActions = @(
    "import_source",
    "expand_story",
    "rewrite_story",
    "accept_story_body",
    "create_story_task",
    "generate_storyboard",
    "repair_storyboard",
    "validate_result",
    "export_result",
    "golden_sample_review"
)

$rulePackIds = New-Object System.Collections.Generic.HashSet[string]
foreach ($pack in @($snapshot.writing_rule_packs) + @($snapshot.director_rule_packs) + @($snapshot.validation_rule_packs)) {
    [void]$rulePackIds.Add([string]$pack.id)
}

$reviewedWikiIds = New-Object System.Collections.Generic.HashSet[string]
Get-ChildItem -LiteralPath (Join-Path $repo "knowledge\reviewed_wiki") -Filter "*.md" -ErrorAction SilentlyContinue | ForEach-Object {
    [void]$reviewedWikiIds.Add($_.BaseName)
}

$mappingWikiIds = New-Object System.Collections.Generic.HashSet[string]
foreach ($item in @($mapping.mappings)) {
    [void]$mappingWikiIds.Add([string]$item.reviewed_wiki_id)
    if ($item.runtime_eligible -and (-not $item.runtime_rule_pack_ids -or $item.runtime_rule_pack_ids.Count -eq 0)) {
        Add-Error "Runtime eligible mapping has no runtime_rule_pack_ids: $($item.mapping_id)"
    }
    $targetFieldPaths = @($item.target_field_paths | ForEach-Object { [string]$_ })
    foreach ($target in @($item.runtime_targets)) {
        $targetName = [string]$target
        $hasTargetPath = $false
        foreach ($path in $targetFieldPaths) {
            if ($path.StartsWith($targetName) -or $path.Contains(".$targetName")) {
                $hasTargetPath = $true
                break
            }
        }
        if (-not $hasTargetPath) {
            Add-Error "Runtime target has no matching target_field_paths entry: $($item.mapping_id) -> $targetName"
        }
    }
    foreach ($packId in @($item.runtime_rule_pack_ids)) {
        if (-not $rulePackIds.Contains([string]$packId)) {
            Add-Error "Mapping references missing rule pack: $($item.mapping_id) -> $packId"
        }
    }
    if ($item.leakage_count -ne 0) {
        Add-Error "Mapping leakage_count is not zero: $($item.mapping_id)"
    }
}

foreach ($pack in @($snapshot.writing_rule_packs) + @($snapshot.director_rule_packs) + @($snapshot.validation_rule_packs)) {
    foreach ($wikiId in @($pack.source_reviewed_wiki_ids)) {
        if (-not $reviewedWikiIds.Contains([string]$wikiId)) {
            Add-Error "Rule pack references missing reviewed wiki file: $($pack.id) -> $wikiId"
        }
        if (-not $mappingWikiIds.Contains([string]$wikiId)) {
            Add-Error "Rule pack reviewed wiki id missing in mapping: $($pack.id) -> $wikiId"
        }
    }
}

foreach ($profile in @($snapshot.duration_profiles)) {
    foreach ($wikiId in @($profile.source_reviewed_wiki_ids)) {
        if (-not $reviewedWikiIds.Contains([string]$wikiId)) {
            Add-Error "Duration profile references missing reviewed wiki file: $($profile.duration) -> $wikiId"
        }
        if (-not $mappingWikiIds.Contains([string]$wikiId)) {
            Add-Error "Duration profile reviewed wiki id missing in mapping: $($profile.duration) -> $wikiId"
        }
    }
}

$allowedDurations = @($snapshot.allowed_durations | ForEach-Object { [int]$_ })
$profileDurations = @($snapshot.duration_profiles | ForEach-Object { [int]$_.duration })
$coverageDurations = @($snapshot.coverage_matrix.duration_profile_coverage | ForEach-Object { [int]$_.duration })
foreach ($duration in $allowedDurations) {
    if ($profileDurations -notcontains $duration) {
        Add-Error "Allowed duration missing duration_profile: $duration"
    }
    if ($coverageDurations -notcontains $duration) {
        Add-Error "Allowed duration missing coverage: $duration"
    }
}

$coveredActions = @($snapshot.coverage_matrix.kb_action_coverage | ForEach-Object { [string]$_.kb_action })
$actionResultActions = @($snapshot.action_results | ForEach-Object { [string]$_.action })
foreach ($action in $requiredActions) {
    if ($coveredActions -notcontains $action) {
        Add-Error "Required action missing coverage: $action"
    }
    if ($actionResultActions -notcontains $action) {
        Add-Error "Required action missing action_result: $action"
    }
}

foreach ($coverage in @($snapshot.coverage_matrix.kb_action_coverage)) {
    if ($coverage.covered -ne $true) {
        Add-Error "Required action coverage is not true: $($coverage.kb_action)"
    }
    if ($coverage.covered -eq $true -and (-not $coverage.rule_pack_ids -or $coverage.rule_pack_ids.Count -eq 0)) {
        Add-Error "Covered action has no rule_pack_ids: $($coverage.kb_action)"
    }
    foreach ($packId in @($coverage.rule_pack_ids)) {
        if (-not $rulePackIds.Contains([string]$packId)) {
            Add-Error "Action coverage references missing rule pack: $($coverage.kb_action) -> $packId"
        }
    }
}

if ($snapshot.runtime_safety.raw_kb_rows_included -ne 0) { Add-Error "raw_kb_rows_included must be 0" }
foreach ($field in @("raw_sample_text_absent", "source_register_absent", "overlay_json_absent", "prompt_body_absent", "raw_graph_absent", "local_paths_absent", "secrets_absent")) {
    if ($snapshot.runtime_safety.$field -ne $true) { Add-Error "runtime_safety.$field must be true" }
}
foreach ($field in @("runtime_graph_lookup_used", "runtime_auto_ingest_used", "runtime_llm_summarize_used")) {
    if ($snapshot.runtime_safety.$field -ne $false) { Add-Error "runtime_safety.$field must be false" }
}

$allowedSafetyFieldNames = New-Object System.Collections.Generic.HashSet[string]
foreach ($name in @(
    "raw_kb_rows_included",
    "raw_sample_text_absent",
    "raw_source_text_absent",
    "raw_kb_rows_absent",
    "source_register_absent",
    "overlay_json_absent",
    "prompt_body_absent",
    "raw_graph_absent",
    "local_paths_absent",
    "secrets_absent",
    "runtime_graph_lookup_used",
    "runtime_auto_ingest_used",
    "runtime_llm_summarize_used",
    "negative_constraints"
)) {
    [void]$allowedSafetyFieldNames.Add($name)
}

$deniedFieldPattern = '^(raw_source_text|raw_kb_rows|prompt_body|source_register|overlay_json|provider_config|api_key|token|secret|credential|local_path|absolute_path)$'
$secretValuePattern = '(sk-[A-Za-z0-9]{12,}|AKIA[0-9A-Z]{12,}|-----BEGIN [A-Z ]+PRIVATE KEY-----|Bearer\s+[A-Za-z0-9._-]{20,}|[A-Za-z]:\\[^"''\r\n]+)'

function Test-JsonNodeSafety($Node, [string]$Path, [string]$File) {
    if ($null -eq $Node) { return }

    if ($Node -is [string]) {
        if ($Node -match $secretValuePattern) {
            Add-Error "Potential raw secret/path value in $File : $Path"
        }
        return
    }

    if ($Node -is [ValueType]) { return }

    if ($Node -is [System.Collections.IEnumerable]) {
        $index = 0
        foreach ($item in $Node) {
            Test-JsonNodeSafety $item "$Path[$index]" $File
            $index += 1
        }
        return
    }

    if ($Node.PSObject.Properties.Count -gt 0) {
        foreach ($property in $Node.PSObject.Properties) {
            $name = [string]$property.Name
            $nextPath = if ($Path) { "$Path.$name" } else { $name }
            if ($name -match $deniedFieldPattern -and -not $allowedSafetyFieldNames.Contains($name)) {
                Add-Error "Denied field name in $File : $nextPath"
            }
            Test-JsonNodeSafety $property.Value $nextPath $File
        }
        return
    }
}

Test-JsonNodeSafety $snapshot "" $snapshotFullPath
Test-JsonNodeSafety $mapping "" $mappingFullPath
Test-JsonNodeSafety $pwaAdapter "" $pwaAdapterFullPath

if ($pwaAdapter.adapter_version -ne "pwa-kb-adapter/v0.2") {
    Add-Error "PWA adapter version must be pwa-kb-adapter/v0.2"
}
if ($pwaAdapter.adapter_status -ne "prototype") {
    Add-Error "Batch 1 PWA adapter sample must remain prototype"
}
if ($pwaAdapter.scene_catalog_status -ne "partial") {
    Add-Error "Batch 1 PWA adapter sample must declare partial scene catalog"
}
if ($pwaAdapter.source_snapshot_version -ne $snapshot.snapshot_version) {
    Add-Error "PWA adapter source_snapshot_version does not match runtime snapshot"
}
if ($pwaAdapter.source_snapshot_hash -ne $snapshot.snapshot_hash) {
    Add-Error "PWA adapter source_snapshot_hash does not match runtime snapshot"
}
if ($pwaAdapter.sanitized_summary_contract.summary_only -ne $true) {
    Add-Error "PWA adapter sanitized summary contract must be summary_only"
}
if ($pwaAdapter.fail_closed.fallback_required -ne $true) {
    Add-Error "PWA adapter must require fail-closed fallback"
}

$pwaSnapshot = $pwaAdapter.pwa_snapshot
foreach ($field in @("snapshotVersion", "sceneTypes", "allowedDurations", "writingRulePacks", "directorRulePacks", "validationRulePacks", "sceneMappings", "durationProfiles")) {
    if ($null -eq $pwaSnapshot.$field) {
        Add-Error "PWA adapter snapshot missing camelCase field: $field"
    }
}
foreach ($snakeField in @("snapshot_version", "scene_types", "allowed_durations", "writing_rule_packs", "director_rule_packs", "validation_rule_packs", "scene_mappings", "duration_profiles")) {
    if ($null -ne $pwaSnapshot.$snakeField) {
        Add-Error "PWA adapter snapshot must not expose snake_case field: $snakeField"
    }
}

$pwaRulePackIds = New-Object System.Collections.Generic.HashSet[string]
foreach ($pack in @($pwaSnapshot.writingRulePacks) + @($pwaSnapshot.directorRulePacks) + @($pwaSnapshot.validationRulePacks)) {
    [void]$pwaRulePackIds.Add([string]$pack.id)
    foreach ($field in @("influenceAxes", "negativeConstraints")) {
        if ($null -eq $pack.$field) {
            Add-Error "PWA rule pack missing camelCase field: $($pack.id) -> $field"
        }
    }
    foreach ($snakeField in @("influence_axes", "negative_constraints", "source_reviewed_wiki_ids")) {
        if ($null -ne $pack.$snakeField) {
            Add-Error "PWA rule pack must not expose KB-only field: $($pack.id) -> $snakeField"
        }
    }
}

foreach ($sceneType in @($pwaSnapshot.sceneTypes)) {
    $sceneId = [string]$sceneType
    $sceneMapping = $pwaSnapshot.sceneMappings.$sceneId
    if ($null -eq $sceneMapping) {
        Add-Error "PWA sceneTypes entry missing sceneMappings entry: $sceneId"
        continue
    }
    foreach ($field in @("writingRulePackIds", "directorRulePackIds", "validationRulePackIds", "sceneProfile", "negativeConstraints", "influenceAxes")) {
        if ($null -eq $sceneMapping.$field) {
            Add-Error "PWA scene mapping missing camelCase field: $sceneId -> $field"
        }
    }
    foreach ($packId in @($sceneMapping.writingRulePackIds) + @($sceneMapping.directorRulePackIds) + @($sceneMapping.validationRulePackIds)) {
        if (-not $pwaRulePackIds.Contains([string]$packId)) {
            Add-Error "PWA scene mapping references missing rule pack: $sceneId -> $packId"
        }
    }
}

foreach ($duration in @($pwaSnapshot.allowedDurations)) {
    $durationValue = [int]$duration
    $profile = @($pwaSnapshot.durationProfiles | Where-Object { [int]$_.duration -eq $durationValue })
    if ($profile.Count -eq 0) {
        Add-Error "PWA allowed duration missing durationProfile: $durationValue"
    }
}

foreach ($file in (Get-ChildItem -LiteralPath (Join-Path $repo "knowledge\reviewed_wiki") -Filter "*.md").FullName) {
    $text = Get-Content -Raw -LiteralPath $file
    if ($text -match $secretValuePattern) {
        Add-Error "Potential raw secret/path value in reviewed wiki: $file"
    }
}

if ($errors.Count -gt 0) {
    $result = [ordered]@{
        status = "failed"
        error_count = $errors.Count
        errors = $errors
    }
    $result | ConvertTo-Json -Depth 6
    exit 1
}

[ordered]@{
    status = "passed"
    reviewed_wiki_count = $reviewedWikiIds.Count
    mapping_count = @($mapping.mappings).Count
    rule_pack_count = $rulePackIds.Count
    duration_count = $allowedDurations.Count
    action_count = $requiredActions.Count
    pwa_adapter_status = [string]$pwaAdapter.adapter_status
    pwa_adapter_scene_catalog_status = [string]$pwaAdapter.scene_catalog_status
} | ConvertTo-Json -Depth 4
