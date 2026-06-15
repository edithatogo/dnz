# Track Specification: Semantic Docs & Conda-forge Packaging

## Overview
This track builds automation linking the Power BI semantic model schema directly into the Astro documentation portal, and implements Conda packaging recipes to distribute Python FFI libraries.

## User Stories / Requirements
- As a developer, I want to auto-extract table metadata and DAX formulas from TMDL files to document the semantic model.
- As a data scientist, I want to install the `dnz-python` client via Conda channels (`pixi` or `conda install dnz`).
- As a CI system, I want automatic verification of the TMDL schema validity on pull requests.

## Technical Constraints
- **Docs Compiler:** Astro Static Site generator reading compiled TMDL Markdown specs.
- **Conda Tooling:** Conda recipe structure and pixi channels.
- **Validation Engine:** `pbi-cli` / semantic schema lint rules.
