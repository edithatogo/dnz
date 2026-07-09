# Track Specification: Source Archive Expansion

## Overview

Expand the `dnz` archive surface so all checked-in source submodules that represent DigitalNZ-originated source material are covered by the public mirror workflows, not just the notebook corpus under `digitalnz/`.

This track treats the source archive as a bundle of source-led artifacts. The initial scope is the existing `digitalnz/` notebook/data archive and the legacy `pydnz/` Python client source snapshot. Both are separately maintained source trees in this repo and both should be represented in the archive metadata and payloads.

## Functional Requirements

- Identify every source submodule or source-led tree that belongs in the archive bundle.
  - `digitalnz/` notebook and data corpus.
  - `pydnz/` legacy Python client source.
- Extend the archive payloads.
  - Hugging Face mirror should include all in-scope source trees.
  - Zenodo publication should include the same source trees.
  - A top-level archive README should describe the combined archive and its parts.
- Keep source artifacts intact.
  - Do not alter the source trees solely for packaging.
  - Exclude Git metadata from the copied archive payload.
- Update the archive-facing documentation.
  - Document which source trees are mirrored.
  - Clarify which items remain code/docs-only and are not archived yet.

## Non-Functional Requirements

- The archive bundle must remain reproducible from checked-in repo content.
- Archive workflows must not require secrets to exist at packaging time unless they actually publish remotely.
- The archive README must not overclaim completeness beyond the scoped source trees.

## Acceptance Criteria

- The HF and Zenodo archive workflows include both `digitalnz/` and `pydnz/`.
- A top-level archive README explains the combined source bundle.
- Archive documentation states the current scope explicitly.
- The workflow changes do not mutate source trees or include Git metadata.

## Out of Scope

- Archiving unrelated runtime, build, or docs-only trees as source bundles.
- Reworking the source submodules themselves beyond archive-safe documentation.
- Changing release binaries or registry submission behavior.
