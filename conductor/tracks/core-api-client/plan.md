# Track Plan: Core API Client

- [ ] Task 1: Define API Models - Declare structural models matching the DigitalNZ API v3 schemas using serde.
- [ ] Task 2: Implement Client Struct - Build the asynchronous `Client` struct utilizing `reqwest` for raw endpoint actions.
- [ ] Task 3: Build Search & Facet Query Builders - Implement builders to cleanly construct API queries (including bounding boxes, sorting, filtering).
- [ ] Task 4: Setup Wiremock Integration Tests - Configure mock servers using `wiremock` to run offline assertions on client logic, verify code quality and push.
