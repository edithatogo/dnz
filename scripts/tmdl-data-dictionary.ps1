param(
    [string]$ModelRoot = "powerbi/semantic-model/DigitalNZ.SemanticModel/definition",
    [string]$OutputPath = "docs/src/content/docs/generated/semantic-model-dictionary.md"
)

$ErrorActionPreference = "Stop"
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$modelPath = Join-Path $repo $ModelRoot
$output = Join-Path $repo $OutputPath

if (-not (Test-Path -LiteralPath $modelPath)) {
    throw "TMDL model path not found: $modelPath"
}

$tablesPath = Join-Path $modelPath "tables"
$tables = Get-ChildItem -LiteralPath $tablesPath -Filter "*.tmdl" | Sort-Object Name
$lines = New-Object System.Collections.Generic.List[string]

$lines.Add("---")
$lines.Add("title: Semantic Model Dictionary")
$lines.Add("description: Generated table and measure inventory for the DigitalNZ Power BI semantic model.")
$lines.Add("---")
$lines.Add("")
$lines.Add("# Semantic Model Dictionary")
$lines.Add("")
$lines.Add("Generated from `powerbi/semantic-model/DigitalNZ.SemanticModel/definition`.")
$lines.Add("")

foreach ($file in $tables) {
    $content = Get-Content -LiteralPath $file.FullName
    $tableLine = $content | Where-Object { $_ -match '^table\s+' } | Select-Object -First 1
    if (-not $tableLine) { continue }

    $tableName = ($tableLine -replace '^table\s+', '').Trim()
    $lines.Add("## $tableName")
    $lines.Add("")

    $description = $content | Where-Object { $_ -match '^\s+description:\s+' } | Select-Object -First 1
    if ($description) {
        $lines.Add(($description -replace '^\s+description:\s+', '').Trim())
        $lines.Add("")
    }

    $columns = @()
    $current = $null
    foreach ($line in $content) {
        if ($line -match '^\s+column\s+(.+)$') {
            if ($current) { $columns += $current }
            $current = [ordered]@{ name = $Matches[1].Trim(); dataType = ""; sourceColumn = "" }
        } elseif ($current -and $line -match '^\s+dataType:\s+(.+)$') {
            $current.dataType = $Matches[1].Trim()
        } elseif ($current -and $line -match '^\s+sourceColumn:\s+(.+)$') {
            $current.sourceColumn = $Matches[1].Trim()
        } elseif ($current -and $line -match '^\s+(measure|partition)\s+') {
            $columns += $current
            $current = $null
        }
    }
    if ($current) { $columns += $current }

    if ($columns.Count -gt 0) {
        $lines.Add("| Column | Data Type | Source Column |")
        $lines.Add("|---|---|---|")
        foreach ($column in $columns) {
            $lines.Add("| $($column.name) | $($column.dataType) | $($column.sourceColumn) |")
        }
        $lines.Add("")
    }

    $measures = @()
    foreach ($line in $content) {
        if ($line -match '^\s+measure\s+(.+?)\s+=\s+(.+)$') {
            $measures += [ordered]@{ name = $Matches[1].Trim(); expression = $Matches[2].Trim() }
        }
    }

    if ($measures.Count -gt 0) {
        $lines.Add("| Measure | Expression |")
        $lines.Add("|---|---|")
        foreach ($measure in $measures) {
            $expression = $measure.expression.Replace("|", "\|")
            $lines.Add("| $($measure.name) | `$expression` |")
        }
        $lines.Add("")
    }
}

$outputDir = Split-Path -Parent $output
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null
Set-Content -LiteralPath $output -Value $lines -Encoding UTF8
Write-Host "Generated $OutputPath"

