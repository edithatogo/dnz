$ErrorActionPreference = "Stop"
$CargoArgs = $args
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
Set-Location -LiteralPath $repo

$isWindows = $PSVersionTable.Platform -eq "Win32NT" -or $env:OS -eq "Windows_NT"
$gnuToolchain = $false
$mingwBin = $null

if ($isWindows) {
    $gnuToolchain = [bool]((& rustup toolchain list 2>$null) | Where-Object { $_ -match "stable-x86_64-pc-windows-gnu" } | Select-Object -First 1)
    $gcc = Get-Command "gcc.exe" -ErrorAction SilentlyContinue
    if ($gcc) {
        $mingwBin = Split-Path -Parent $gcc.Source
    } else {
        $scoopMingw = Join-Path $env:USERPROFILE "scoop\apps\mingw\current\bin"
        if (Test-Path -LiteralPath (Join-Path $scoopMingw "gcc.exe")) {
            $mingwBin = $scoopMingw
        }
    }
}

if ($isWindows -and $gnuToolchain -and $mingwBin) {
    $env:PATH = "$mingwBin;$env:PATH"
    if (-not $env:CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR = Join-Path ([System.IO.Path]::GetTempPath()) "dnz-target-gnu"
    }
    & rustup run stable-x86_64-pc-windows-gnu cargo @CargoArgs
} else {
    & cargo @CargoArgs
}

exit $LASTEXITCODE
