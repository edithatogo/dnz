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
$isWindows = $IsWindows -or $env:OS -eq "Windows_NT"
if ($isWindows) {
    $msvcToolchain = (& rustup toolchain list 2>$null | Where-Object { $_ -match "stable-x86_64-pc-windows-msvc" } | Select-Object -First 1)
    $link = Get-Command "link.exe" -ErrorAction SilentlyContinue
    $hasRealMsvcLink = $link -and ($link.Source -notmatch "\\scoop\\apps\\git\\" -and $link.Source -notmatch "\\git\\current\\usr\\bin\\link\.exe$")
    $kernelLibInEnv = $false
    if ($env:LIB) {
        foreach ($dir in ($env:LIB -split ';')) {
            if ($dir -and (Test-Path -LiteralPath (Join-Path $dir "kernel32.lib"))) {
                $kernelLibInEnv = $true
                break
            }
        }
    }
    $windowsKitLib = Get-ChildItem -Directory "C:\Program Files (x86)\Windows Kits\10\Lib" -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending |
        ForEach-Object { Join-Path $_.FullName "um\x64\kernel32.lib" } |
        Where-Object { Test-Path -LiteralPath $_ } |
        Select-Object -First 1
    if ($msvcToolchain -and $hasRealMsvcLink -and ($kernelLibInEnv -or $windowsKitLib)) {
        $cargoArgs += "+stable-x86_64-pc-windows-msvc"
    } else {
        $gnuToolchain = (& rustup toolchain list 2>$null | Where-Object { $_ -match "stable-x86_64-pc-windows-gnu" } | Select-Object -First 1)
        $gnuProfiler = Join-Path $env:USERPROFILE ".rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\lib"
        $gnuHasProfiler = $gnuToolchain -and (Test-Path -LiteralPath $gnuProfiler) -and (Get-ChildItem -LiteralPath $gnuProfiler -Filter "libprofiler_builtins-*.rlib" -ErrorAction SilentlyContinue)
        if (-not $gnuHasProfiler) {
            $gnuStatus = if ($gnuToolchain) { "installed but lacks libprofiler_builtins" } else { "not installed" }
            throw "Local Windows coverage requires a Rust toolchain with profiler_builtins. stable-x86_64-pc-windows-gnu is $gnuStatus; stable-x86_64-pc-windows-msvc has profiler support but this host lacks a usable Visual Studio linker/Windows SDK import libraries. Run this coverage gate in CI or install the missing Windows coverage toolchain prerequisites."
        }
        $cargoArgs += "+stable-x86_64-pc-windows-gnu"
    }
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
