# Track Plan: Power BI Semantic Model Integration

- [x] Task 15.1: Scaffold the base TMDL model structure.
  - *Evidence:* Added `powerbi/semantic-model/DigitalNZ.SemanticModel/definition/` with model, expression, and table TMDL files.
  - *Commit:* `feat(track-15): task 15.1 - scaffold TMDL semantic model project`
- [x] Task 15.2: Configure Power Query M source queries loading Frictionless exports.
  - *Evidence:* Added `RecordsCsvPath`, `CitationsCsvPath`, `VectorClustersCsvPath`, `LoadCsv`, and import partitions for Frictionless CSV resources.
  - *Commit:* `feat(track-15): task 15.2 - add M source expressions for Frictionless files`
- [x] Task 15.3: Define DAX measures (Total Records, Unique Citations, and Time Distributions).
  - *Evidence:* Added `Measures.tmdl` with total records, records with dates, unique citations, citation coverage, content partner/category counts, records per year, vector cluster count, and average similarity score.
  - *Commit:* `feat(track-15): task 15.3 - implement analytical DAX measures`
- [x] Task 15.4: Set up semantic relationships and star schema mappings in the model.
  - *Evidence:* Added `Records` fact table, `DimDate`, `Citations`, `VectorClusters`, and model relationships on `Year` and `RecordId`.
  - *Commit:* `feat(track-15): task 15.4 - configure model relationships and hierarchy`
