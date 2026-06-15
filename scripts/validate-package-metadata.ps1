param(
    [string[]]$Crates = @("dnz-core", "dnz-cli", "dnz-mcp"),
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"

function Assert-True {
    param(
        [bool]$Condition,
        [string]$Message
    )

    if (-not $Condition) {
        throw $Message
    }
}

function Get-PackageMetadata {
    param([string]$PackageName)

    $metadataJson = cargo metadata --no-deps --format-version 1
    $metadata = $metadataJson | ConvertFrom-Json
    $package = $metadata.packages | Where-Object { $_.name -eq $PackageName } | Select-Object -First 1
    Assert-True ($null -ne $package) "Cargo package '$PackageName' was not found in workspace metadata."
    return $package
}

foreach ($crate in $Crates) {
    Write-Host "Validating Cargo package metadata for $crate"
    $package = Get-PackageMetadata -PackageName $crate

    Assert-True ($package.license -eq "MIT") "$crate must declare MIT license metadata."
    Assert-True ($package.repository -eq "https://github.com/edithatogo/dnz") "$crate must declare repository metadata."
    Assert-True ($package.homepage -eq "https://github.com/edithatogo/dnz") "$crate must declare homepage metadata."
    Assert-True ($package.readme -eq "../../README.md") "$crate must package the workspace README."

    if ($crate -ne "dnz-core") {
        $coreDependency = $package.dependencies | Where-Object { $_.name -eq "dnz-core" } | Select-Object -First 1
        Assert-True ($null -ne $coreDependency) "$crate must depend on dnz-core."
        Assert-True ($coreDependency.req -eq "^0.1.0") "$crate must declare a versioned dnz-core dependency for crates.io packaging."
    }

    $packageArgs = @("package", "-p", $crate, "--list")
    if ($AllowDirty) {
        $packageArgs += "--allow-dirty"
    }
    $packageList = cargo @packageArgs
    Assert-True ($packageList -contains "Cargo.toml") "$crate package list must include Cargo.toml."
    Assert-True ($packageList -contains "README.md") "$crate package list must include README.md."
}

$pyprojectPath = Join-Path $PSScriptRoot "..\crates\dnz-python\pyproject.toml"
$pyproject = Get-Content -LiteralPath $pyprojectPath -Raw
Assert-True ($pyproject -match 'license\s*=\s*\{\s*text\s*=\s*"MIT"\s*\}') "Python pyproject must declare MIT license metadata."
Assert-True ($pyproject -match 'Homepage\s*=\s*"https://github.com/edithatogo/dnz"') "Python pyproject must declare Homepage URL."
Assert-True ($pyproject -match 'Repository\s*=\s*"https://github.com/edithatogo/dnz"') "Python pyproject must declare Repository URL."

$condaMetaPath = Join-Path $PSScriptRoot "..\conda-recipe\meta.yaml"
$condaMeta = Get-Content -LiteralPath $condaMetaPath -Raw
Assert-True ($condaMeta -match 'license:\s*MIT') "Conda recipe must declare MIT license metadata."
Assert-True ($condaMeta -match 'license_file:\s*LICENSE') "Conda recipe must reference the root LICENSE file."

Write-Host "Package metadata validation passed."
