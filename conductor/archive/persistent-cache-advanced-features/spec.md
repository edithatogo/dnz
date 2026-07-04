# Track Specification: Persistent Cache & Advanced API Features

## Overview
This track implements robust performance improvements including SQLite-based persistent caching (replacing simple thread-safe in-memory cache), dynamic downloads for Candle embeddings models, and structured JSON-LD logging for debugging.

## User Stories / Requirements
- As a CLI/MCP user, I want search queries to be cached locally in a persistent database across sessions so that subsequent identical runs do not hit rate limits.
- As a RAG pipeline integration, I want the system to automatically retrieve embedding models if they are not already installed locally.
- As a system administrator, I want to toggle structured JSON logging on CLI/Daemon output for unified log analysis.

## Technical Constraints
- **Caching Database:** SQLite (`rusqlite` or `sqlx` sqlite driver).
- **Embeddings Pipeline:** Local HF downloader integration in `candle` logic.
- **Structured Logging:** `tracing-subscriber` JSON formatting feature.
