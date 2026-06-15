# Track Specification: RAG Context Digests

## Overview
This track introduces structured layout templates designed to optimize DigitalNZ search results for LLM consumption. It builds citation managers and semantic reducers that format data into compact, context-dense XML and Markdown structures.

## User Stories / Requirements
- As an LLM agent, I want search results structured in token-efficient XML tags (e.g. `<record id="...">...</record>`) to prevent context bloating.
- As a researcher, I want auto-generated bibliographies and research timelines compiled directly from search sets.
- As a developer, I want duplicate records and overlapping texts automatically pruned before output generation.

## Technical Constraints
- Formatting templates built using static layouts.
- Integrates text deduplication routines (e.g. MinHash or strict metadata equality).
