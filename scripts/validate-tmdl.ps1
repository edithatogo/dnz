param(
    [string]$ModelRoot = "powerbi/semantic-model/DigitalNZ.SemanticModel/definition"
)

$ErrorActionPreference = "Stop"
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$modelPath = Join-Path $repo $ModelRoot

if (-not (Test-Path -LiteralPath $modelPath)) {
    throw "TMDL model path not found: $modelPath"
}

$required = @(
    "model.tmdl",
    "expressions.tmdl",
    "tables/Records.tmdl",
    "tables/DimDate.tmdl",
    "tables/Citations.tmdl",
    "tables/VectorClusters.tmdl",
    "tables/Measures.tmdl"
)

foreach ($relative in $required) {
    $path = Join-Path $modelPath $relative
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Required TMDL file missing: $relative"
    }
}

$tableFiles = Get-ChildItem -LiteralPath (Join-Path $modelPath "tables") -Filter "*.tmdl"
foreach ($file in $tableFiles) {
    $text = Get-Content -LiteralPath $file.FullName -Raw
    if ($text -notmatch '(?m)^table\s+') {
        throw "Missing table declaration: $($file.FullName)"
    }
}

$modelText = Get-Content -LiteralPath (Join-Path $modelPath "model.tmdl") -Raw
foreach ($relationship in @("Records_DimDate_Year", "Citations_Records_RecordId", "VectorClusters_Records_RecordId")) {
    if ($modelText -notmatch [regex]::Escape($relationship)) {
        throw "Missing relationship: $relationship"
    }
}

Write-Host "TMDL scaffold validation passed for $ModelRoot"

