//! Polars DataFrame integration for DigitalNZ heritage records.

use crate::models::Record;
use polars_core::prelude::*;

/// Extension trait to convert lists of Records directly to Polars DataFrames.
pub trait IntoDataFrame {
    /// Convert search records into a Polars DataFrame.
    fn into_dataframe(self) -> Result<DataFrame, PolarsError>;
}

impl IntoDataFrame for Vec<Record> {
    fn into_dataframe(self) -> Result<DataFrame, PolarsError> {
        let height = self.len();
        let mut ids = Vec::with_capacity(self.len());
        let mut titles = Vec::with_capacity(self.len());
        let mut descriptions = Vec::with_capacity(self.len());
        let mut content_partners = Vec::with_capacity(self.len());
        let mut categories = Vec::with_capacity(self.len());

        for rec in self {
            ids.push(rec.id);
            titles.push(rec.title);
            descriptions.push(rec.description.unwrap_or_default());
            content_partners.push(
                rec.content_partner
                    .map(|v| v.join(", "))
                    .unwrap_or_default(),
            );
            categories.push(rec.category.map(|v| v.join(", ")).unwrap_or_default());
        }

        let id_series = Series::new("id".into(), ids);
        let title_series = Series::new("title".into(), titles);
        let desc_series = Series::new("description".into(), descriptions);
        let partner_series = Series::new("content_partner".into(), content_partners);
        let category_series = Series::new("category".into(), categories);

        DataFrame::new(
            height,
            vec![
                id_series.into(),
                title_series.into(),
                desc_series.into(),
                partner_series.into(),
                category_series.into(),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_records_to_expected_dataframe_shape() {
        let records = vec![
            Record {
                id: "1".to_string(),
                title: "Kauri".to_string(),
                description: Some("Tree".to_string()),
                content_partner: Some(vec!["Te Papa".to_string(), "DNZ".to_string()]),
                category: Some(vec!["Images".to_string()]),
                ..Record::default()
            },
            Record {
                id: "2".to_string(),
                title: "Tui".to_string(),
                ..Record::default()
            },
        ];

        let frame = records.into_dataframe().unwrap();
        assert_eq!(frame.height(), 2);
        assert_eq!(frame.width(), 5);
        assert_eq!(
            frame.get_column_names(),
            ["id", "title", "description", "content_partner", "category"]
        );
    }

    #[test]
    fn converts_empty_record_list_to_empty_dataframe_with_schema() {
        let frame = Vec::<Record>::new().into_dataframe().unwrap();
        assert_eq!(frame.height(), 0);
        assert_eq!(frame.width(), 5);
    }
}
