param(
    [string]$TargetDir = (Join-Path ([System.IO.Path]::GetTempPath()) "dnz-target-wheel")
)

$ErrorActionPreference = "Stop"
if ($PSVersionTable.PSVersion.Major -ge 7) {
    $PSNativeCommandUseErrorActionPreference = $true
}
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location -LiteralPath $repo

if (-not $env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR = $TargetDir
}

Write-Host "Building Python wheel with maturin..."
maturin build --release --manifest-path crates/dnz-python/Cargo.toml
if ($LASTEXITCODE -ne 0) {
    throw "maturin build failed with exit code $LASTEXITCODE"
}

Write-Host "Checking wheel metadata with twine..."
$wheelDir = Join-Path $repo "target\wheels"
if (Test-Path -LiteralPath $wheelDir) {
    $wheels = Get-ChildItem -Path $wheelDir -Filter "*.whl"
    if ($wheels.Count -eq 0) {
        # Check CARGO_TARGET_DIR for wheels
        $altWheelDir = Join-Path $TargetDir "wheels"
        if (Test-Path -LiteralPath $altWheelDir) {
            $wheels = Get-ChildItem -Path $altWheelDir -Filter "*.whl"
        }
    }
    if ($wheels.Count -eq 0) {
        Write-Warning "No .whl files found in target/wheels; twine check skipped."
        exit 0
    }
    python -m twine check $wheels.FullName
    if ($LASTEXITCODE -ne 0) {
        throw "twine check failed with exit code $LASTEXITCODE"
    }
    Write-Host "Python wheel metadata validation passed."
} else {
    Write-Warning "target/wheels directory not found; twine check skipped."
    exit 0
}
