//! Context optimization and RAG formatting routines for DigitalNZ records.

use crate::models::Record;
use std::collections::HashSet;

/// Prune near-duplicate records based on exact title matching or ID uniqueness.
pub fn deduplicate_records(records: &[Record]) -> Vec<Record> {
    let mut seen_titles = HashSet::new();
    let mut unique_records = Vec::new();

    for rec in records {
        // Standardize title for comparison
        let normalized_title = rec.title.to_lowercase().trim().to_string();
        if !normalized_title.is_empty() && !seen_titles.contains(&normalized_title) {
            seen_titles.insert(normalized_title);
            unique_records.push(rec.clone());
        }
    }

    unique_records
}

/// Serialize records into a token-efficient XML format optimized for LLM RAG pipelines.
pub fn to_rag_xml(records: &[Record]) -> String {
    let mut output = String::new();
    output.push_str("<context_documents>\n");

    for rec in records {
        let partners = rec
            .content_partner
            .as_ref()
            .map(|v| v.join(", "))
            .unwrap_or_default();
        output.push_str(&format!("  <document id=\"{}\">\n", rec.id));
        output.push_str(&format!("    <title>{}</title>\n", rec.title));
        if !partners.is_empty() {
            output.push_str(&format!(
                "    <content_partner>{}</content_partner>\n",
                partners
            ));
        }
        if let Some(desc) = &rec.description {
            output.push_str(&format!("    <description>{}</description>\n", desc.trim()));
        }
        if let Some(dates) = &rec.date {
            output.push_str(&format!("    <dates>{}</dates>\n", dates.join(", ")));
        }
        output.push_str("  </document>\n");
    }

    output.push_str("</context_documents>\n");
    output
}

/// Compile a chronological timeline of records.
pub fn generate_chronological_timeline(records: &[Record]) -> String {
    let mut timeline_entries: Vec<(String, String)> = Vec::new();

    for rec in records {
        if let Some(dates) = &rec.date {
            if let Some(first_date) = dates.first() {
                timeline_entries.push((first_date.clone(), rec.title.clone()));
            }
        }
    }

    // Sort entries chronologically by date string
    timeline_entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut markdown = String::new();
    markdown.push_str("# Historical Research Timeline\n\n");
    if timeline_entries.is_empty() {
        markdown.push_str("*No dates associated with these records.*\n");
        return markdown;
    }

    for (date, title) in timeline_entries {
        markdown.push_str(&format!("- **{}**: {}\n", date, title));
    }

    markdown
}

/// Generate MLA/APA style reference citations for heritage archives.
pub fn generate_citations(records: &[Record]) -> String {
    let mut output = String::new();
    output.push_str("# Heritage Document Citations\n\n");

    for rec in records {
        let partner = rec
            .content_partner
            .as_ref()
            .and_then(|v| v.first())
            .map_or("Unknown Partner", |s| s.as_str());
        let date_str = rec
            .date
            .as_ref()
            .and_then(|v| v.first())
            .map_or("n.d.", |s| s.as_str());
        let source = rec.source_url.as_deref().unwrap_or("digitalnz.org");

        output.push_str(&format!(
            "- \"{}\" ({}), *{}*. Retrieved from: <{}>\n",
            rec.title, date_str, partner, source
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str, title: &str, date: Option<&str>) -> Record {
        Record {
            id: id.to_string(),
            title: title.to_string(),
            description: Some("Description".to_string()),
            content_partner: Some(vec!["Partner".to_string()]),
            date: date.map(|d| vec![d.to_string()]),
            source_url: Some(format!("https://example.test/{id}")),
            ..Record::default()
        }
    }

    #[test]
    fn deduplicate_records_keeps_first_title_case_insensitively() {
        let records = vec![
            record("1", "Kauri Tree", Some("1900")),
            record("2", "kauri tree", Some("1901")),
            record("3", "Tui", Some("1902")),
        ];

        let deduped = deduplicate_records(&records);
        assert_eq!(deduped.len(), 2);
        assert_eq!(deduped[0].id, "1");
        assert_eq!(deduped[1].id, "3");
    }

    #[test]
    fn rag_xml_includes_core_record_fields() {
        let xml = to_rag_xml(&[record("1", "Kauri", Some("1900"))]);

        assert!(xml.contains("<document id=\"1\">"));
        assert!(xml.contains("<title>Kauri</title>"));
        assert!(xml.contains("<content_partner>Partner</content_partner>"));
        assert!(xml.contains("<dates>1900</dates>"));
    }

    #[test]
    fn chronological_timeline_sorts_by_date_text() {
        let timeline = generate_chronological_timeline(&[
            record("2", "Later", Some("1902")),
            record("1", "Earlier", Some("1901")),
        ]);

        let earlier = timeline.find("Earlier").unwrap();
        let later = timeline.find("Later").unwrap();
        assert!(earlier < later);
    }

    #[test]
    fn citations_include_partner_date_and_source() {
        let citations = generate_citations(&[record("1", "Kauri", Some("1900"))]);

        assert!(citations.contains("\"Kauri\" (1900), *Partner*"));
        assert!(citations.contains("<https://example.test/1>"));
    }
}
