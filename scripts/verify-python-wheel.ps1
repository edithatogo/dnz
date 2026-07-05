param(
    [string]$TargetDir = (Join-Path ([System.IO.Path]::GetTempPath()) "dnz-target-wheel"),
    [string]$Python,
    [switch]$SkipBuild
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

$isWindows = $PSVersionTable.Platform -eq "Win32NT" -or $env:OS -eq "Windows_NT"
$mingwBin = $null
if ($isWindows -and $env:USERPROFILE) {
    $mingwBin = Join-Path $env:USERPROFILE "scoop\apps\mingw\current\bin"
    if (Test-Path -LiteralPath $mingwBin) {
        $env:PATH = "$mingwBin;$env:PATH"
    }
}

$gnuToolchain = $null
if ($isWindows) {
    $gnuToolchain = (& rustup toolchain list 2>$null | Where-Object { $_ -match "stable-x86_64-pc-windows-gnu" } | Select-Object -First 1)
}
if ($isWindows -and $gnuToolchain -and $mingwBin -and (Test-Path -LiteralPath $mingwBin)) {
    $env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
}

if (-not $env:CARGO_BUILD_JOBS) {
    $env:CARGO_BUILD_JOBS = "1"
}

function Invoke-Tool {
    param(
        [string]$Name,
        [string[]]$Arguments
    )

    if (Get-Command $Name -ErrorAction SilentlyContinue) {
        & $Name @Arguments
    } elseif (Get-Command "uvx" -ErrorAction SilentlyContinue) {
        & uvx $Name @Arguments
    } else {
        throw "$Name is not on PATH and uvx is unavailable"
    }

    if ($LASTEXITCODE -ne 0) {
        throw "$Name $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

if (-not $SkipBuild) {
    Write-Host "Building Python wheel with maturin..."
    $maturinArgs = @("build", "--release", "--manifest-path", "crates/dnz-python/Cargo.toml")
    if (-not $Python) {
        $candidatePythons = @(
            (Join-Path $repo ".pixi\envs\default\python.exe"),
            (Join-Path $env:LOCALAPPDATA "Programs\Python\Python312\python.exe")
        )
        foreach ($candidate in $candidatePythons) {
            if (Test-Path -LiteralPath $candidate) {
                $Python = $candidate
                break
            }
        }
    }
    if ($Python) {
        $maturinArgs += @("-i", $Python)
    }
    Invoke-Tool "maturin" $maturinArgs
}

Write-Host "Checking wheel metadata with twine..."
$wheelDir = Join-Path $repo "target\wheels"
$altWheelDir = Join-Path $TargetDir "wheels"
$wheels = @()
if (Test-Path -LiteralPath $wheelDir) {
    $wheels = Get-ChildItem -Path $wheelDir -Filter "*.whl"
}
if ($wheels.Count -eq 0 -and (Test-Path -LiteralPath $altWheelDir)) {
    $wheels = Get-ChildItem -Path $altWheelDir -Filter "*.whl"
}
if ($wheels.Count -eq 0) {
    Write-Warning "No .whl files found in target/wheels or $altWheelDir; twine check skipped."
    exit 0
}
Invoke-Tool "twine" (@("check") + $wheels.FullName)
Write-Host "Python wheel metadata validation passed."
