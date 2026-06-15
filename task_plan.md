# Swarm Task Plan: DigitalNZ Integration Hub (12 Tracks Execution)

This plan maps out the step-by-step tasks to implement the 12 project tracks under strict quality and automation gates.

## Phase 1: Foundations & Infrastructure (Tracks 1 - 4)
- [ ] Task 1.1: Complete environment setup (Cargo/Pixi targets, ruff lint config). -> **Cline** & **Codex**
- [ ] Task 1.2: Complete CLI skeleton code with version flags. -> **Cline**
- [ ] Task 1.3: Complete MCP JSON-RPC loop template over stdin/stdout. -> **Cline**
- [ ] Task 1.4: Implement core API client (Search/Facets schemas, async HTTP Client). -> **Codex**
- [ ] Task 1.5: Wire up CLI commands to the core client wrapper. -> **Cline**
- [ ] Task 1.6: Connect MCP server tools to API methods. -> **Cline**
- [ ] **Phase 1 Validation:** Validate clippy limits, formatting, and standard build test runs. -> **Quality_Validator**

## Phase 2: Testing & Performance (Tracks 5 - 7)
- [ ] Task 2.1: Write wiremock integrations for fully offline hermetic tests. -> **Codex**
- [ ] Task 2.2: Implement proptest suites verifying pagination boundaries. -> **Codex**
- [ ] Task 2.3: Integrate cargo-mutants checks in test pipeline. -> **Codex**
- [ ] Task 2.4: Integrate Polars dataframes to structure and sort records. -> **Codex**
- [ ] Task 2.5: Implement tracing diagnostics outputting to stderr. -> **Cline**
- [ ] Task 2.6: Configure cargo-dist binaries compiling multi-platform targets. -> **Cline**
- [ ] **Phase 2 Validation:** Confirm test coverage is >90% and clippy has 0 warnings. -> **Quality_Validator**

## Phase 3: Advanced Features (Tracks 8 - 12)
- [ ] Task 3.1: Build local vector database using SQLite/Sled. -> **Codex**
- [ ] Task 3.2: Implement Candle text embedding generation pipeline. -> **Codex**
- [ ] Task 3.3: Implement XML & Markdown RAG-optimized formatters. -> **Cline**
- [ ] Task 3.4: Build density-based query splitter (Agentic Autopilot). -> **Codex**
- [ ] Task 3.5: Wrap Rust core API into Python using pyo3 & maturin. -> **Codex** & **Cline**
- [ ] Task 3.6: Develop automated Open Science metadata exports (OSF/Zenodo). -> **Codex**
- [ ] **Phase 3 Validation:** Perform final test suites verification, checking that CI builds cleanly. -> **Quality_Validator**

## Phase 4: Production Hardening & Diagnostics (Tracks 13 - 14)
- [ ] Task 4.1: Configure GitHub Actions release secrets for crates.io and PyPI. -> **Quality_Validator**
- [ ] Task 4.2: Verify line coverage exceeds 90% target locally using `cargo tarpaulin`. -> **Quality_Validator**
- [ ] Task 4.3: Profile and optimize hot-paths in vector similarity search. -> **Codex**
- [ ] Task 4.4: Integrate cargo-audit checks in pixi tasks and CI builds. -> **Quality_Validator**
- [ ] Task 4.5: Create workspace doctor script diagnosing PATH, targets, and disk limits. -> **Quality_Validator**
- [ ] Task 4.6: Set up automated local Git pre-commit verification hooks. -> **Quality_Validator**
- [ ] **Phase 4 Validation:** Verify package dry-runs finish successfully and diagnostic checks pass. -> **Quality_Validator**


## Phase 5: Semantic Reporting & Business Intelligence (Track 15)
- [ ] Task 5.1: Scaffold Power BI TMDL model metadata project. -> **Codex**
- [ ] Task 5.2: Configure Power Query M source queries referencing Frictionless CSV outputs. -> **Codex**
- [ ] Task 5.3: Implement DAX analysis measures for citation counting and clustering. -> **Codex**
- [ ] Task 5.4: Define model relationships, hierarchies, and star schema routing. -> **Codex**
- [ ] **Phase 5 Validation:** Compile and validate TMDL semantic model schemas. -> **Quality_Validator**

## Phase 6: Automated Packaging & Documentation Sync (Track 16)
- [ ] Task 6.1: Develop data dictionary extraction parser converting TMDL schema to Markdown. -> **Codex**
- [ ] Task 6.2: Integrate semantic model dictionaries into the Astro layout architecture dashboard. -> **Cline**
- [ ] Task 6.3: Configure Conda distribution recipes for Python FFI binary packages. -> **Quality_Validator**
- [ ] Task 6.4: Add pull request checking step executing TMDL compilation validation in CI. -> **Quality_Validator**
- [ ] **Phase 6 Validation:** Run dry-runs of Conda packaging pipelines and verify docs sync cleanly. -> **Quality_Validator**



