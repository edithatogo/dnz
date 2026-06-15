# Code Style Guides: DigitalNZ Integration Hub (Rust Core)

## 1. Rust Quality & Tooling Guidelines

### Linting & Formatting
- **Formatting:** Enforced via `cargo fmt`.
- **Linting:** Enforced via `cargo clippy`. Root module configurations must forbid warnings:
  ```rust
  #![deny(clippy::all, clippy::pedantic, clippy::nursery)]
  #![deny(missing_docs)]
  ```

### Data Validation & Domain Modeling
- **Type Safety:** Ensure strict type constraints. Replace raw primitive types with validated newtypes using `nutype` or the `validator` crate.
- **Serialization:** Apply `serde` serialization annotations directly to response structures mapping to the DigitalNZ schema.

## 2. Advanced Testing Matrix (Target: >90% Coverage)

### Unit & Integration Testing
- Create isolated unit tests in module source files.
- Place cross-module integration tests in the `tests/` directory to verify CLI-to-API and MCP-to-API behaviors.
- **Hermetic HTTP Testing:** All integration tests communicating with the DigitalNZ API must utilize `wiremock` to prevent external network dependencies.

### Property-Based Testing
- Integrate the `proptest` or `quickcheck` crates.
- Use property tests to validate query parsing, bounding box arithmetic, and pagination edge cases with randomized inputs.

### Mutation Testing
- Use `cargo-mutants` to perform mutation testing.
- All code branches must resist injected mutants. Track plans must check that mutant resistance is verified before merging.

### Performance Profiling & Microbenchmarks
- Use `divan` or `criterion` for benchmarking performance-critical operations.
- Run `cargo-flamegraph` to profile and identify execution hot spots.

### Coverage Enforcement
- Use `cargo-tarpaulin` or `grcov` to measure code coverage.
- Target coverage must exceed **90%** of all lines across the workspace. CI gates must fail if coverage drops below this threshold.

## 3. Observability Standards
- Use the `tracing` crate to instrument asynchronous tasks.
- Avoid printing debug logs or errors directly to `stdout` within library modules or MCP logic. All diagnostics must be emitted as tracing events targeting `stderr`.
