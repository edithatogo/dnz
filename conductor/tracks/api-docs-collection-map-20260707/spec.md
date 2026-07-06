# Track Specification: API Docs & Collection Map

## Overview

Map the repository’s API documentation into the Astro/Starlight docs surface and add a repo-local inventory of the major DigitalNZ collections exposed through the API.

The repository does not have a top-level `api/` directory. The current API-adjacent documentation lives across `README.md`, `pydnz/`, `digitalnz/`, `docs/src/content/docs/`, the Rust crate docs, and generated collection/facet artifacts. This track turns that scattered material into a canonical documentation map inside `docs/` and a collection inventory that readers can use to understand the major collection groups behind the API.

## Functional Requirements

- Inventory all API-related documentation sources in the repo.
  - Include the root `README.md`, `pydnz/README.md`, `digitalnz/` notebooks and README, and the existing Astro docs pages.
  - Identify which sources are canonical, which are historical examples, and which should be linked rather than copied.
- Produce a documentation map for the repo’s API surfaces.
  - Map each source document to its destination in the docs site or to a retained source-artifact location.
  - Cover CLI, MCP, Python, core library, API usage, and collection/facet reference material.
- Add a major-collections inventory for DigitalNZ API-backed content.
  - Derive the inventory from repository data such as `digitalnz/facets/collections_by_partner.csv`, `digitalnz/facets/usage_by_collection_and_partner.csv`, `digitalnz/open_collections_digitalnz.html`, and the notebooks that generated those summaries.
  - Include the major collections/partners by record volume and note the source artifact used for each entry.
- Surface the documentation map and collection inventory in the docs site.
  - Add docs pages or generated docs entries under the existing Astro/Starlight content tree.
  - Link the new pages from the docs index and any relevant overview pages.
- Keep the mapping reproducible.
  - The collection inventory should be derived from repo-local artifacts, not live API calls.
  - The source-to-destination map should be explicit enough for future contributors to update without guesswork.

## Non-Functional Requirements

- Documentation changes must not alter runtime behavior in Rust, Python, or MCP code.
- The collection inventory should be deterministic from checked-in inputs.
- The docs pages should remain concise and navigable for readers who need a stable entrypoint into the API surface.
- Existing notebooks and CSVs must remain available as source artifacts.

## Acceptance Criteria

- A canonical API documentation map exists in the docs tree.
- A major-collections inventory exists in the docs tree and is derived from repository data.
- The docs index links to the new API documentation and collection map pages.
- The source-to-destination mapping identifies the main API-related documentation surfaces in this repo.
- The collection inventory names the major DigitalNZ collection groups visible in the repository’s facet data.
- Docs validation passes for the changed content.

## Out of Scope

- Changing the DigitalNZ API client behavior.
- Refreshing collection counts from live API calls.
- Reworking the notebook source material itself unless a docs migration needs a small linkage update.
- Publishing the docs site outside this repository.
