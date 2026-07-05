param(
    [string]$Version = "0.1.0",
    [string]$Tag = "v0.1.0",
    [string]$OutputDir = "dist",
    [switch]$AllowDirty
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")
Set-Location $repoRoot

if (-not $AllowDirty) {
    $status = git status --porcelain
    if ($status) {
        throw "Working tree is dirty. Commit or stash changes, or rerun with -AllowDirty."
    }
}

$outputPath = Join-Path $repoRoot $OutputDir
$bundleRoot = Join-Path $outputPath "dnz-mcpb"
$serverDir = Join-Path $bundleRoot "server"
$bundlePath = Join-Path $outputPath "dnz-mcp-$Version.mcpb"

if (Test-Path -LiteralPath $bundleRoot) {
    Remove-Item -LiteralPath $bundleRoot -Recurse -Force
}
New-Item -ItemType Directory -Path $serverDir -Force | Out-Null

$assets = @(
    "dnz-mcp-linux-x86_64",
    "dnz-mcp-macos-x86_64",
    "dnz-mcp-windows-x86_64.exe"
)

foreach ($asset in $assets) {
    gh release download $Tag --pattern $asset --dir $serverDir --clobber
}

$manifestTemplate = Join-Path $repoRoot "registry/mcpb/manifest.template.json"
$manifestPath = Join-Path $bundleRoot "manifest.json"
$manifest = Get-Content -Raw -LiteralPath $manifestTemplate
$manifest = $manifest.Replace('"version": "0.1.0"', ('"version": "' + $Version + '"'))
[System.IO.File]::WriteAllText($manifestPath, $manifest, [System.Text.UTF8Encoding]::new($false))

if (Test-Path -LiteralPath $bundlePath) {
    Remove-Item -LiteralPath $bundlePath -Force
}

$zipPath = "$bundlePath.zip"
if (Test-Path -LiteralPath $zipPath) {
    Remove-Item -LiteralPath $zipPath -Force
}

Compress-Archive -Path (Join-Path $bundleRoot "*") -DestinationPath $zipPath -Force
Move-Item -LiteralPath $zipPath -Destination $bundlePath -Force

npx -y @anthropic-ai/mcpb clean $bundlePath

$sha256 = (Get-FileHash -LiteralPath $bundlePath -Algorithm SHA256).Hash.ToLowerInvariant()
$metadata = [ordered]@{
    version = $Version
    tag = $Tag
    file = (Resolve-Path -LiteralPath $bundlePath).Path
    sha256 = $sha256
    assets = $assets
}
$metadataPath = Join-Path $outputPath "dnz-mcp-$Version.mcpb.sha256.json"
$metadataJson = $metadata | ConvertTo-Json -Depth 4
[System.IO.File]::WriteAllText($metadataPath, $metadataJson, [System.Text.UTF8Encoding]::new($false))

Write-Host "MCPB bundle: $bundlePath"
Write-Host "SHA-256: $sha256"
