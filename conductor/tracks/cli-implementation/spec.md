# Track Specification: CLI Implementation

## Overview
This track implements the command-line interface tool (`dnz-cli`) using the `clap` crate. It links the console arguments to the Core API client and formats response outputs (JSON or Markdown tables).

## User Stories / Requirements
- As a CLI user, I want to execute search and facet harvests directly from the terminal.
- As a pipeline developer, I want to receive CLI outputs as stdout JSON for stream piping.
- As a user, I want query parameters (e.g. search term, category filters, page size) mapped to CLI flags.

## Technical Constraints
- Command line arguments parsed using `clap` (derive interface).
- Outputs serialized to Markdown tables (default) or formatted JSON.
