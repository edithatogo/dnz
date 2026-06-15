# DigitalNZ Power BI Semantic Model

This folder contains the source-controlled TMDL scaffold for the DigitalNZ analytical model.

Expected Frictionless export inputs:

- `exports/frictionless/records.csv`
- `exports/frictionless/citations.csv`
- `exports/frictionless/vector_clusters.csv`

The model is designed around a simple star schema:

- `Records` is the fact table for harvested DigitalNZ records.
- `DimDate` supports chronological distributions via `Records[Year]`.
- `Citations` stores one citation row per record/reference.
- `VectorClusters` stores semantic cluster assignments per record.
- `Measures` holds reusable DAX measures.

When connected to a Power BI Desktop model, import these files with `pbi database import-tmdl` or copy the TMDL definitions into the model after `pbi connect`.

