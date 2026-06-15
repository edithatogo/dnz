param(
    [int]$FailUnderLines = 90,
    [switch]$Html,
    [string]$TargetDir = (Join-Path ([System.IO.Path]::GetTempPath()) "dnz-target-coverage")
)

$ErrorActionPreference = "Stop"
if ($PSVersionTable.PSVersion.Major -ge 7) {
    $PSNativeCommandUseErrorActionPreference = $true
}
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location -LiteralPath $repo

$mingwBin = Join-Path $env:USERPROFILE "scoop\apps\mingw\current\bin"
if (Test-Path -LiteralPath $mingwBin) {
    $env:PATH = "$mingwBin;$env:PATH"
}

if (-not $env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR = $TargetDir
}

$cargoArgs = @()
if ((& rustup toolchain list 2>$null | Where-Object { $_ -match "stable-x86_64-pc-windows-gnu" } | Select-Object -First 1)) {
    $cargoArgs += "+stable-x86_64-pc-windows-gnu"
}

$cargoArgs += @(
    "llvm-cov",
    "--workspace",
    "--all-features",
    "--fail-under-lines",
    "$FailUnderLines"
)

if ($Html) {
    $cargoArgs += "--html"
}

& cargo @cargoArgs
if ($LASTEXITCODE -ne 0) {
    throw "cargo $($cargoArgs -join ' ') failed with exit code $LASTEXITCODE"
}
