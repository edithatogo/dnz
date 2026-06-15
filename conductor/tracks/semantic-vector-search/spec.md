# Track Specification: Semantic Vector Search

## Overview
This track implements a local vector database and embedding generator inside the client. It lets users embed harvested DigitalNZ documents locally using lightweight models (e.g. via `candle`) and execute hybrid search combining API facets with local vector similarity.

## User Stories / Requirements
- As a researcher, I want to execute semantic searches over previously harvested records offline.
- As a developer, I want a tool to generate and store embeddings from DigitalNZ text fields (metadata/titles/descriptions).
- As a system, I want a hybrid search scoring mechanism combining API text matches and local cosine similarity.

## Technical Constraints
- Model loading and computation driven by `candle` (minimalist ML framework for Rust) or `ort` (ONNX Runtime bindings).
- Thread-safe local database storage (e.g., using `sqlite` or `sled` for vector/metadata persistence).
