# Track Plan: Packaging & Registry Submission

- [ ] Task 1: Setup Pixi & Release Builds - Define `pixi.toml` for task management and keep the GitHub Actions release matrix aligned with supported cross-platform binaries.
- [ ] Task 2: Schema Exporter - Create an automated helper tool inside the crate to dump current MCP schemas and OpenAPI specifications to static JSON files.
- [ ] Task 3: Build Registry Manifests - Write manifest definitions and documentation files tailored to MCP, Cline, GitHub Copilot, and generic agentic skill registries.
- [ ] Task 4: Complete CD Pipeline - Establish the GitHub Actions workflow that compiles binaries, attaches them to a GitHub Release, and publishes the schemas, and check CI result.
