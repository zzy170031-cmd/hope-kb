param(
  [string]$RepoRoot = "E:\codex\hope-kb"
)

$ErrorActionPreference = "Stop"

$manifestPath = Join-Path $RepoRoot "seed\v0.1\manifest.json"
$importMapPath = Join-Path $RepoRoot "seed\v0.1\import_map.json"

$manifest = Get-Content -Raw -Encoding utf8 $manifestPath | ConvertFrom-Json
$importMap = Get-Content -Raw -Encoding utf8 $importMapPath | ConvertFrom-Json

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

$bundleHash = $manifest.content_hash -replace '^bundle-sha256:', ''
$bytes = New-Object System.Collections.Generic.List[byte]
foreach ($rel in $manifest.bundle_order) {
  $path = Join-Path $RepoRoot ($rel -replace '/', '\')
  if (!(Test-Path $path)) {
    throw "Missing bundle_order file: $rel"
  }
  $bytes.AddRange([System.IO.File]::ReadAllBytes($path))
}
$sha = [System.Security.Cryptography.SHA256]::Create()
$actualHash = [System.BitConverter]::ToString($sha.ComputeHash($bytes.ToArray())).Replace('-', '').ToLower()
if ($actualHash -ne $bundleHash) {
  throw "Bundle hash mismatch: expected $bundleHash, actual $actualHash"
}

$results | Format-Table -AutoSize
Write-Output ""
Write-Output "Seed bundle validation passed."
