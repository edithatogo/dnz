//! Source-grounded data-quality and rights/reuse summaries.

use crate::models::Record;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataQualityReport {
    pub total_records: usize,
    pub unique_ids: usize,
    pub duplicate_ids: usize,
    pub missing_titles: usize,
    pub missing_source_urls: usize,
    pub missing_rights_metadata: usize,
    pub caveat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RightsReuseAudit {
    pub total_records: usize,
    pub records_with_rights: usize,
    pub records_with_rights_url: usize,
    pub records_with_usage_statement: usize,
    pub commercial_use_true: usize,
    pub commercial_use_false: usize,
    pub commercial_use_unknown: usize,
    pub caveat: String,
}

/// Summarize observable completeness and duplicate-ID signals only.
pub fn assess_data_quality(records: &[Record]) -> DataQualityReport {
    let mut ids = HashSet::new();
    for record in records {
        ids.insert(record.id.as_str());
    }
    let unique_ids = ids.len();
    DataQualityReport {
        total_records: records.len(),
        unique_ids,
        duplicate_ids: records.len().saturating_sub(unique_ids),
        missing_titles: records
            .iter()
            .filter(|record| record.title.trim().is_empty())
            .count(),
        missing_source_urls: records
            .iter()
            .filter(|record| record.source_url.as_deref().unwrap_or("").trim().is_empty())
            .count(),
        missing_rights_metadata: records
            .iter()
            .filter(|record| record.rights.is_none() && record.usage.is_none())
            .count(),
        caveat: "Completeness signals describe supplied records only; they do not establish provider-wide coverage or data correctness.".to_string(),
    }
}

/// Summarize rights fields without making legal conclusions about reuse.
pub fn audit_rights_reuse(records: &[Record]) -> RightsReuseAudit {
    let commercial_use_true = records
        .iter()
        .filter(|record| record.is_commercial_use == Some(true))
        .count();
    let commercial_use_false = records
        .iter()
        .filter(|record| record.is_commercial_use == Some(false))
        .count();
    RightsReuseAudit {
        total_records: records.len(),
        records_with_rights: records.iter().filter(|record| record.rights.is_some()).count(),
        records_with_rights_url: records
            .iter()
            .filter(|record| record.rights_url.is_some())
            .count(),
        records_with_usage_statement: records.iter().filter(|record| record.usage.is_some()).count(),
        commercial_use_true,
        commercial_use_false,
        commercial_use_unknown: records.len() - commercial_use_true - commercial_use_false,
        caveat: "This is a metadata audit, not legal advice and not a determination of copyright, licence, or permitted reuse.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str, title: &str) -> Record {
        Record {
            id: id.to_string(),
            title: title.to_string(),
            ..Record::default()
        }
    }

    #[test]
    fn quality_report_counts_observable_gaps_and_duplicates() {
        let mut second = record("1", "");
        second.source_url = Some("https://example.test/1".into());
        let report = assess_data_quality(&[record("1", "One"), second]);
        assert_eq!(
            (
                report.total_records,
                report.unique_ids,
                report.duplicate_ids
            ),
            (2, 1, 1)
        );
        assert_eq!(report.missing_titles, 1);
        assert!(report.caveat.contains("supplied records"));
    }

    #[test]
    fn rights_audit_preserves_unknown_and_avoids_legal_claims() {
        let mut commercial = record("1", "One");
        commercial.rights = Some("CC BY".into());
        commercial.rights_url = Some("https://example.test/rights".into());
        commercial.is_commercial_use = Some(true);
        let mut restricted = record("2", "Two");
        restricted.is_commercial_use = Some(false);
        let audit = audit_rights_reuse(&[commercial, restricted, record("3", "Three")]);
        assert_eq!(
            (audit.records_with_rights, audit.commercial_use_true),
            (1, 1)
        );
        assert_eq!(audit.commercial_use_unknown, 1);
        assert!(audit.caveat.contains("not legal advice"));
    }
}
