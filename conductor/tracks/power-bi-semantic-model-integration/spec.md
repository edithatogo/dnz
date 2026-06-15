# Track Specification: Power BI Semantic Model Integration

## Overview
This track builds a Power BI semantic model (using the modern TMDL layout format) that references the exported Frictionless Data Package from the DigitalNZ Integration Hub, exposing tables, columns, and DAX measures for analytics.

## User Stories / Requirements
- As a business analyst, I want to load the Frictionless Data Package output directly into a Power BI model using Power Query M expressions.
- As a report builder, I want pre-defined DAX measures calculating citation counts, chronological distributions, and vector clusters.
- As a developer, I want to manage this semantic model inside the Git repository using version-controlled TMDL files.

## Technical Constraints
- **Model Format:** Tabular Model Definition Language (TMDL).
- **Data Connector:** Power Query M querying exported Frictionless CSV/JSON.
- **DAX Calculations:** Citation analysis and timeline calculations.
- **tooling:** `pbi-cli` / `power-bi-modeling` schema orchestration.
