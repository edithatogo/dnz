---
title: Semantic Model Dictionary
description: Generated table and measure inventory for the DigitalNZ Power BI semantic model.
---

# Semantic Model Dictionary

Generated from powerbi/semantic-model/DigitalNZ.SemanticModel/definition.

## Citations

Citation rows generated from DigitalNZ records.

| Column | Data Type | Source Column |
|---|---|---|
| CitationKey | string | citation_key |
| RecordId | string | record_id |
| CitationText | string | citation_text |
| SourceUrl | string | source_url |

## DimDate

Year-level date dimension for chronological DigitalNZ analysis.

| Column | Data Type | Source Column |
|---|---|---|
| Year | int64 | Year |
| Decade | string | Decade |

## Measures

Analytical DAX measures for DigitalNZ reporting.

| Measure | Expression |
|---|---|
| Total Records | $expression |
| Records With Dates | $expression |
| Unique Citations | $expression |
| Citation Coverage % | $expression |
| Distinct Content Partners | $expression |
| Distinct Categories | $expression |
| Records Per Year | $expression |
| Vector Cluster Count | $expression |
| Average Similarity Score | $expression |

## Records

Harvested DigitalNZ records from the Frictionless records resource.

| Column | Data Type | Source Column |
|---|---|---|
| RecordId | string | id |
| Title | string | title |
| Description | string | description |
| ContentPartner | string | content_partner |
| Category | string | category |
| DateText | string | date |
| Year | int64 | year |
| DisplayUrl | string | display_url |
| SourceUrl | string | source_url |

## VectorClusters

Semantic vector cluster assignments for DigitalNZ records.

| Column | Data Type | Source Column |
|---|---|---|
| RecordId | string | record_id |
| ClusterId | string | cluster_id |
| SimilarityScore | double | similarity_score |

