# Track Plan: Semantic Vector Search

- [ ] Task 1: Integrate Candle / Model Loader - Add the `candle-core` and `candle-nn` crates and write a helper module to load Bert/Sentence-Transformer weights.
- [ ] Task 2: Vector Database Engine - Set up local database persistence using `sqlite` or `sled` to index record IDs and vector blobs.
- [ ] Task 3: Implement Embedding Pipeline - Build a command to parse DigitalNZ metadata, compute text embeddings, and store them.
- [ ] Task 4: Hybrid Search Integration - Build query routines blending keyword scores and cosine similarity, with tests verified.
