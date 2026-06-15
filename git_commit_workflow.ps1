# Automated Git Commit & Push Workflow Script for DigitalNZ integration
# Run this script from a working terminal to stage and commit all implemented tasks.

Write-Host "Starting Git Workflow Automation..." -ForegroundColor Cyan

# Ensure we are in the correct repository location
$repoDir = "C:\Users\60217257\OneDrive - Flinders\repos\legal-nz\dnz"
Set-Location $repoDir
Write-Host "Working Directory: $PWD"

# --- HELPER FUNCTION FOR COMMITS ---
function Commit-Task {
    param(
        [string]$Message,
        [string[]]$Files
    )
    Write-Host "Staging files for commit: $Message" -ForegroundColor Yellow
    foreach ($file in $Files) {
        if (Test-Path $file) {
            git add $file
        }
    }
    git commit -m $Message
}

# --- TRACK 1 ---
Write-Host "`n=== TRACK 1: Scaffold CLI & MCP Skeleton ===" -ForegroundColor Green
Commit-Task -Message "chore(track-1): task 1.1 - configure Cargo and Pixi workspace" -Files @("Cargo.toml", "pixi.toml", ".gitignore")
Commit-Task -Message "feat(track-1): task 1.2 - clap CLI entrypoint and help flags" -Files @("crates/dnz-cli/Cargo.toml", "crates/dnz-cli/src/main.rs")
Commit-Task -Message "feat(track-1): task 1.3 - async JSON-RPC MCP loop skeleton" -Files @("crates/dnz-mcp/Cargo.toml", "crates/dnz-mcp/src/main.rs")
Write-Host "Pushing Track 1..." -ForegroundColor Cyan
git push origin main

# --- TRACK 2 ---
Write-Host "`n=== TRACK 2: Core API Client ===" -ForegroundColor Green
Commit-Task -Message "feat(track-2): task 2.1 - implement DigitalNZ API v3 serialization models" -Files @("crates/dnz-core/src/models.rs")
Commit-Task -Message "feat(track-2): task 2.2 - implement HTTP Client with backoff retries" -Files @("crates/dnz-core/src/client.rs", "crates/dnz-core/Cargo.toml", "crates/dnz-core/src/lib.rs")
Write-Host "Pushing Track 2..." -ForegroundColor Cyan
git push origin main

# --- TRACK 3 ---
Write-Host "`n=== TRACK 3: CLI Implementation ===" -ForegroundColor Green
Commit-Task -Message "feat(track-3): task 3.1 - bind search and facets commands in CLI" -Files @("crates/dnz-cli/src/lib.rs")
Write-Host "Pushing Track 3..." -ForegroundColor Cyan
git push origin main

# --- TRACK 4 ---
Write-Host "`n=== TRACK 4: MCP Server Features ===" -ForegroundColor Green
Commit-Task -Message "feat(track-4): task 4.1 - register digitalnz tools on MCP server" -Files @("crates/dnz-mcp/src/main.rs")
Commit-Task -Message "feat(track-4): task 4.2 - support schema spec exports for MCP" -Files @("crates/dnz-mcp/mcp-manifest.json")
Write-Host "Pushing Track 4..." -ForegroundColor Cyan
git push origin main

# === PHASE 1 REVIEW GATE ===
Write-Host "`n=== PHASE 1 REVIEW GATE ===" -ForegroundColor Cyan
Write-Host "Phase 1 complete. Verify workspace builds cleanly."

# --- TRACK 5 ---
Write-Host "`n=== TRACK 5: Robustness and Advanced Testing ===" -ForegroundColor Green
Commit-Task -Message "test(track-5): task 5.1 - add offline wiremock integration tests" -Files @("crates/dnz-core/tests/client_tests.rs")
Commit-Task -Message "test(track-5): task 5.2 - add property-based tests for query boundaries" -Files @("crates/dnz-core/tests/property_tests.rs")
Write-Host "Pushing Track 5..." -ForegroundColor Cyan
git push origin main

# --- TRACK 6 ---
Write-Host "`n=== TRACK 6: Performance Profiling and Hardening ===" -ForegroundColor Green
Commit-Task -Message "perf(track-6): task 6.2 - configure criterion benchmark suites" -Files @("crates/dnz-core/benches/benchmarks.rs")
Write-Host "Pushing Track 6..." -ForegroundColor Cyan
git push origin main

# --- TRACK 7 ---
Write-Host "`n=== TRACK 7: Packaging and Registry Submission ===" -ForegroundColor Green
Commit-Task -Message "chore(track-7): task 7.2 - add GitHub Actions testing and release workflows" -Files @(".github/workflows/ci.yml", ".github/workflows/release.yml")
Write-Host "Pushing Track 7..." -ForegroundColor Cyan
git push origin main

# === PHASE 2 REVIEW GATE ===
Write-Host "`n=== PHASE 2 REVIEW GATE ===" -ForegroundColor Cyan

# --- TRACK 8 ---
Write-Host "`n=== TRACK 8: Semantic Vector Search ===" -ForegroundColor Green
Commit-Task -Message "feat(track-8): task 8.1 - candle local embedding pipeline" -Files @("crates/dnz-core/src/vector.rs")
Write-Host "Pushing Track 8..." -ForegroundColor Cyan
git push origin main

# --- TRACK 9 ---
Write-Host "`n=== TRACK 9: RAG Context Digests ===" -ForegroundColor Green
Commit-Task -Message "feat(track-9): task 9.1 - add XML and Markdown RAG digest formatters" -Files @("crates/dnz-core/src/digest.rs")
Write-Host "Pushing Track 9..." -ForegroundColor Cyan
git push origin main

# --- TRACK 10 ---
Write-Host "`n=== TRACK 10: Agentic Query Autopilot ===" -ForegroundColor Green
Commit-Task -Message "feat(track-10): task 10.1 - crawler and query splitter autopilot" -Files @("crates/dnz-core/src/autopilot.rs")
Write-Host "Pushing Track 10..." -ForegroundColor Cyan
git push origin main

# --- TRACK 11 ---
Write-Host "`n=== TRACK 11: Python FFI Bindings ===" -ForegroundColor Green
Commit-Task -Message "feat(track-11): task 11.1 - set up PyO3 bindings and pyproject.toml" -Files @("crates/dnz-python/Cargo.toml", "crates/dnz-python/pyproject.toml")
Commit-Task -Message "feat(track-11): task 11.2 - map core client to python classes" -Files @("crates/dnz-python/src/lib.rs")
Write-Host "Pushing Track 11..." -ForegroundColor Cyan
git push origin main

# --- TRACK 12 ---
Write-Host "`n=== TRACK 12: Open Science Export ===" -ForegroundColor Green
Commit-Task -Message "feat(track-12): task 12.1 - frictionless data package exports" -Files @("crates/dnz-core/src/export.rs", "crates/dnz-core/src/dataframe.rs")
Write-Host "Pushing Track 12..." -ForegroundColor Cyan
git push origin main

# Commit remaining files (docs, plans, etc.)
Write-Host "`n=== COMMITTING REMAINING FILES ===" -ForegroundColor Green
git add .
git commit -m "docs: finalize task plan, progress summaries, and documentation dashboard"
git push origin main

Write-Host "`nGit Workflow Execution Complete!" -ForegroundColor Green
