# Track Specification: Core API Client

## Overview
This track implements the core DigitalNZ API client in Rust. It wraps the DigitalNZ REST API endpoints (Search and Facets) using async HTTP requests and strong serde-based serialization, establishing the programmatic boundary for the project.

## User Stories / Requirements
- As a developer, I want a strongly-typed client to query the DigitalNZ API (Search, Geosearch, Facets, and sorting).
- As a client, I want API responses automatically parsed into robust Rust structs.
- As a system, I want pagination boundaries and API rate-limiting handled transparently.
- As a tester, I want integration tests to run offline against `wiremock` mock APIs.

## Technical Constraints
- Built using `reqwest` (asynchronous) and `serde`.
- Use `wiremock` for offline mock responses in integration tests.
- Target unit and integration test coverage >90%.
- Enforce strict clippy check validation.
