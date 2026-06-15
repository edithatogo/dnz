param(
    [switch]$Strict
)

$ErrorActionPreference = "Stop"
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$checks = New-Object System.Collections.Generic.List[object]

function Add-Check {
    param(
        [string]$Name,
        [bool]$Ok,
        [string]$Detail
    )
    $checks.Add([pscustomobject]@{
        name = $Name
        ok = $Ok
        detail = $Detail
    })
}

Set-Location -LiteralPath $repo

$pathHasSpaces = $repo.Contains(" ")
$targetDir = if ($env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR
} elseif ($pathHasSpaces) {
    Join-Path ([System.IO.Path]::GetTempPath()) "dnz-target-doctor"
} else {
    Join-Path $repo "target"
}
$targetOutsideWorkspace = -not ((Resolve-Path -LiteralPath (Split-Path -Parent $targetDir) -ErrorAction SilentlyContinue).Path -like "$repo*")
Add-Check "workspace_path_spaces" ((-not $pathHasSpaces) -or $targetOutsideWorkspace) $(if ($pathHasSpaces -and $targetOutsideWorkspace) { "$repo (spaces present; target outside workspace: $targetDir)" } else { $repo })

$probeDir = Join-Path $targetDir ".dnz-doctor"
$probeFile = Join-Path $probeDir "write-test.tmp"
try {
    New-Item -ItemType Directory -Force -Path $probeDir | Out-Null
    Set-Content -LiteralPath $probeFile -Value "ok" -NoNewline
    try {
        Remove-Item -LiteralPath $probeFile -Force -ErrorAction Stop
        if (Test-Path -LiteralPath $probeDir) {
            Remove-Item -LiteralPath $probeDir -Force -ErrorAction SilentlyContinue
        }
        Add-Check "target_writable" $true $targetDir
    } catch {
        Add-Check "target_writable" $true "$targetDir (write ok; cleanup warning: $($_.Exception.Message))"
    }
} catch {
    Add-Check "target_writable" $false "$targetDir :: $($_.Exception.Message)"
}

foreach ($tool in @("cargo", "rustc", "git")) {
    $command = Get-Command $tool -ErrorAction SilentlyContinue
    Add-Check "$($tool)_on_path" ([bool]$command) $(if ($command) { $command.Source } else { "missing" })
}

$gnuToolchain = (& rustup toolchain list 2>$null) | Where-Object { $_ -match "stable-x86_64-pc-windows-gnu" } | Select-Object -First 1
$mingwGcc = Get-Command "gcc.exe" -ErrorAction SilentlyContinue
if (-not $mingwGcc) {
    $scoopMingw = Join-Path $env:USERPROFILE "scoop\apps\mingw\current\bin\gcc.exe"
    if (Test-Path -LiteralPath $scoopMingw) {
        $mingwGcc = [pscustomobject]@{ Source = $scoopMingw }
    }
}
Add-Check "gnu_rust_toolchain" ([bool]$gnuToolchain) $(if ($gnuToolchain) { $gnuToolchain } else { "missing stable-x86_64-pc-windows-gnu" })
Add-Check "mingw_gcc_available" ([bool]$mingwGcc) $(if ($mingwGcc) { $mingwGcc.Source } else { "missing gcc.exe" })
$gnuBuildRoute = [bool]($gnuToolchain -and $mingwGcc)

$link = Get-Command "link.exe" -ErrorAction SilentlyContinue
if ($link) {
    $linkSource = $link.Source
    $looksLikeGitLink = $linkSource -match "\\git\\current\\usr\\bin\\link\.exe$" -or $linkSource -match "\\scoop\\apps\\git\\"
    Add-Check "msvc_linker_on_path" ((-not $looksLikeGitLink) -or $gnuBuildRoute) $(if ($looksLikeGitLink -and $gnuBuildRoute) { "$linkSource (Git link.exe; bypassed by GNU Rust toolchain)" } else { $linkSource })
} else {
    Add-Check "msvc_linker_on_path" $gnuBuildRoute $(if ($gnuBuildRoute) { "missing; bypassed by GNU Rust toolchain" } else { "missing" })
}

$rustcSysroot = (& rustc --print sysroot 2>$null)
$rustLld = if ($rustcSysroot) {
    Join-Path $rustcSysroot "lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe"
} else {
    $null
}
Add-Check "rust_lld_available" ($rustLld -and (Test-Path -LiteralPath $rustLld)) $(if ($rustLld) { $rustLld } else { "rustc sysroot unavailable" })

function Test-LibContains {
    param([string]$FileName)
    if (-not $env:LIB) { return $false }
    foreach ($dir in ($env:LIB -split ';')) {
        if ($dir -and (Test-Path -LiteralPath (Join-Path $dir $FileName))) {
            return $true
        }
    }
    return $false
}

$kernelLibInEnv = Test-LibContains "kernel32.lib"
$windowsKitLibs = Get-ChildItem -Directory "C:\Program Files (x86)\Windows Kits\10\Lib" -ErrorAction SilentlyContinue |
    Sort-Object Name -Descending |
    ForEach-Object {
        Join-Path $_.FullName "um\x64\kernel32.lib"
    } |
    Where-Object { Test-Path -LiteralPath $_ } |
    Select-Object -First 1

$hasWindowsSdkLibs = $kernelLibInEnv -or [bool]$windowsKitLibs
Add-Check "windows_sdk_libs" ($hasWindowsSdkLibs -or $gnuBuildRoute) $(if ($kernelLibInEnv) { "kernel32.lib found via LIB" } elseif ($windowsKitLibs) { $windowsKitLibs } elseif ($gnuBuildRoute) { "kernel32.lib not found; bypassed by GNU Rust toolchain" } else { "kernel32.lib not found via LIB or standard Windows Kits path" })
Add-Check "windows_rust_build_route" (($hasWindowsSdkLibs -and $link -and -not $looksLikeGitLink) -or $gnuBuildRoute) $(if ($gnuBuildRoute) { "use: PATH=$($mingwGcc.Source | Split-Path -Parent); cargo +stable-x86_64-pc-windows-gnu ..." } elseif ($hasWindowsSdkLibs -and $link -and -not $looksLikeGitLink) { "MSVC linker and Windows SDK available" } else { "no working MSVC or GNU Windows build route found" })

$driveRoot = (Get-Location).Drive.Root
$driveInfo = [System.IO.DriveInfo]::GetDrives() | Where-Object { $_.Name -eq $driveRoot } | Select-Object -First 1
$freeBytes = if ($driveInfo) { $driveInfo.AvailableFreeSpace } else { (Get-PSDrive -Name ((Get-Location).Drive.Name)).Free }
$freeGb = [math]::Round($freeBytes / 1GB, 2)
Add-Check "disk_free_gb" ($freeGb -ge 5) "$freeGb GB free"

$checks | Format-Table -AutoSize

$failed = @($checks | Where-Object { -not $_.ok })
if ($Strict -and $failed.Count -gt 0) {
    Write-Error "workspace doctor failed $($failed.Count) check(s)"
    exit 1
}
