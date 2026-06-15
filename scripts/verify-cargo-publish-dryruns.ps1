param(
    [string]$TargetDir = (Join-Path ([System.IO.Path]::GetTempPath()) "dnz-target-publish-dryrun"),
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"
if ($PSVersionTable.PSVersion.Major -ge 7) {
    $PSNativeCommandUseErrorActionPreference = $true
}
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location -LiteralPath $repo

function Invoke-Cargo {
    param([string[]]$Arguments)

    $toolchain = (& rustup toolchain list 2>$null | Where-Object { $_ -match "stable-x86_64-pc-windows-gnu" } | Select-Object -First 1)
    if ($toolchain) {
        & cargo "+stable-x86_64-pc-windows-gnu" @Arguments
    } else {
        & cargo @Arguments
    }
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

$mingwBin = Join-Path $env:USERPROFILE "scoop\apps\mingw\current\bin"
if (Test-Path -LiteralPath $mingwBin) {
    $env:PATH = "$mingwBin;$env:PATH"
}

if (-not $env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR = $TargetDir
}

$dirtyArg = @()
if ($AllowDirty) {
    $dirtyArg += "--allow-dirty"
}

$coreArgs = @("publish", "-p", "dnz-core", "--dry-run") + $dirtyArg
Invoke-Cargo $coreArgs

$localCorePatch = "patch.crates-io.dnz-core.path='crates/dnz-core'"
$cliArgs = @("package", "-p", "dnz-cli", "--config", $localCorePatch) + $dirtyArg
$mcpArgs = @("package", "-p", "dnz-mcp", "--config", $localCorePatch) + $dirtyArg
Invoke-Cargo $cliArgs
Invoke-Cargo $mcpArgs
