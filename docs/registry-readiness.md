# DigitalNZ Dataset Registry Readiness

Status: `repository_ready_external_gates_pending`

Roadmap: `digitalnz_dataset_registry_readiness_20260721`

- Parent issue: [#28](https://github.com/edithatogo/dnz/issues/28)
- Rights and provenance: [#29](https://github.com/edithatogo/dnz/issues/29)
- Metadata/inventory DOI: [#30](https://github.com/edithatogo/dnz/issues/30)
- Hugging Face/Croissant: [#31](https://github.com/edithatogo/dnz/issues/31)

## Current contract

DigitalNZ records are third-party catalogue metadata and rights statements. The integration hub may publish derived metadata and provenance, but it does not assert rights over source objects or their media. Record-level `rights_basis` and provenance evidence are required before any public dataset packaging.

The repository-side release target is a metadata/inventory DOI, not an unqualified dump of DigitalNZ media. Hugging Face/Croissant publication is conditional on a stable derived schema, source links, rights fields, and exact manifest provenance.

## External boundary

This document does not claim a minted DOI, completed Software Heritage archival record, or public Hugging Face publication. Those states require authoritative external evidence.
