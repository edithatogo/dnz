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
Add-Check "workspace_path_spaces" (-not $pathHasSpaces) $repo

$targetDir = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $repo "target" }
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

$link = Get-Command "link.exe" -ErrorAction SilentlyContinue
if ($link) {
    $linkSource = $link.Source
    $looksLikeGitLink = $linkSource -match "\\git\\current\\usr\\bin\\link\.exe$" -or $linkSource -match "\\scoop\\apps\\git\\"
    Add-Check "msvc_linker_on_path" (-not $looksLikeGitLink) $linkSource
} else {
    Add-Check "msvc_linker_on_path" $false "missing"
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

Add-Check "windows_sdk_libs" ($kernelLibInEnv -or [bool]$windowsKitLibs) $(if ($kernelLibInEnv) { "kernel32.lib found via LIB" } elseif ($windowsKitLibs) { $windowsKitLibs } else { "kernel32.lib not found via LIB or standard Windows Kits path" })

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
