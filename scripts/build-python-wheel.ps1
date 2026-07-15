param(
    [switch]$Release = $true
)

$ErrorActionPreference = "Stop"
$env:PATH = "$env:USERPROFILE\scoop\apps\mingw\current\bin;$env:PATH"
$env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
$env:CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu"
$env:CARGO = Join-Path (Get-Location) "scripts\cargo-gnu.cmd"
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP "dnz-target-maturin"
$env:CARGO_BUILD_JOBS = "1"
$env:PYO3_PYTHON = Join-Path (Get-Location) ".pixi\envs\default\python.exe"
$env:CARGO_PROFILE_RELEASE_LTO = "false"
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "16"

$arguments = @("build", "--target", "x86_64-pc-windows-gnu", "--manifest-path", "crates/dnz-python/Cargo.toml", "--out", "dist")
if ($Release) {
    $arguments += "--release"
}

& maturin @arguments
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
