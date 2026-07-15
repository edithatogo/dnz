param(
    [switch]$RunLive,
    [string]$BaseUrl = "https://api.digitalnz.org/v3/records.json"
)

$ErrorActionPreference = "Stop"

if (-not $RunLive) {
    Write-Host "Live DigitalNZ smoke test skipped. Pass -RunLive explicitly."
    exit 0
}

$headers = @{}
if ($env:DIGITALNZ_API_KEY) {
    $headers["Authentication-Token"] = $env:DIGITALNZ_API_KEY
}
$uri = "$BaseUrl?text=kiwi&per_page=1"
try {
    $response = Invoke-WebRequest -Uri $uri -Headers $headers -Method Get -TimeoutSec 30
    if ($response.StatusCode -ne 200) {
        throw "DigitalNZ smoke request returned HTTP $($response.StatusCode)."
    }
    $payload = $response.Content | ConvertFrom-Json
    if ($null -eq $payload.search) {
        throw "DigitalNZ smoke response did not contain the expected search envelope."
    }
    Write-Host "DigitalNZ live smoke passed: HTTP 200 and search envelope present."
} catch {
    $status = $_.Exception.Response.StatusCode.value__
    if ($status) {
        throw "DigitalNZ live smoke failed with HTTP $status. Response body and credentials were not logged."
    }
    throw "DigitalNZ live smoke failed: network or response validation error. Credentials were not logged."
}
