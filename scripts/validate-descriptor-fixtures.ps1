param(
  [Parameter(Mandatory = $true)]
  [string]$RepoRoot,

  [string]$FixtureRoot = "tools/descriptor-validator/fixtures"
)

$ErrorActionPreference = "Stop"

function Stop-WithCode {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Message,

    [Parameter(Mandatory = $true)]
    [int]$Code
  )

  [Console]::Error.WriteLine($Message)
  exit $Code
}

function Convert-ToRepoRelativePath {
  param(
    [Parameter(Mandatory = $true)]
    [string]$PathValue,

    [Parameter(Mandatory = $true)]
    [string]$ResolvedRepoRoot
  )

  if ([System.IO.Path]::IsPathRooted($PathValue)) {
    $resolvedPath = [System.IO.Path]::GetFullPath($PathValue)
  }
  else {
    $resolvedPath = [System.IO.Path]::GetFullPath((Join-Path $ResolvedRepoRoot $PathValue))
  }

  $repoPrefix = $ResolvedRepoRoot.TrimEnd('\') + '\'

  if (!$resolvedPath.StartsWith($repoPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
    Stop-WithCode -Message "FixtureRoot must be inside RepoRoot." -Code 2
  }

  $repoUri = New-Object System.Uri($repoPrefix)
  $fixtureUri = New-Object System.Uri($resolvedPath)
  return [System.Uri]::UnescapeDataString($repoUri.MakeRelativeUri($fixtureUri).ToString())
}

if ([string]::IsNullOrWhiteSpace($RepoRoot)) {
  Stop-WithCode -Message "RepoRoot is required." -Code 2
}

if ([string]::IsNullOrWhiteSpace($FixtureRoot)) {
  Stop-WithCode -Message "FixtureRoot is required." -Code 2
}

$resolvedRepoRoot = [System.IO.Path]::GetFullPath($RepoRoot)
if (!(Test-Path -LiteralPath $resolvedRepoRoot -PathType Container)) {
  Stop-WithCode -Message "RepoRoot must be an existing directory." -Code 2
}

$relativeFixtureRoot = Convert-ToRepoRelativePath -PathValue $FixtureRoot -ResolvedRepoRoot $resolvedRepoRoot

Push-Location -LiteralPath $resolvedRepoRoot
try {
  & cargo +1.95.0 run --manifest-path "tools/descriptor-validator/Cargo.toml" -- --matrix $relativeFixtureRoot
  exit $LASTEXITCODE
}
finally {
  Pop-Location
}
