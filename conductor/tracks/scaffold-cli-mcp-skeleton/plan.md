# Track Plan: Scaffold CLI & MCP Skeleton (Rust Core)

- [ ] Task 1: Environment & Workspace Setup - Initialize Cargo and Pixi configuration structures (`pixi.toml` and `Cargo.toml`), and set up linting rules.
- [ ] Task 2: Implement CLI Entry Point - Set up a base binary using `clap` to process help parameters and initialize a basic tracing subscriber.
- [ ] Task 3: Implement MCP Server Skeleton - Set up the asynchronous JSON-RPC protocol loop over stdin/stdout with diagnostic logging directed to stderr.
- [ ] Task 4: Setup Cargo Tests & CI Checks - Implement verify tests in `tests/` verifying CLI invocation and MCP boot checks, and check formatting.
