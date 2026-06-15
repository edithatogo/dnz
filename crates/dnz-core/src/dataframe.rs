//! Polars DataFrame integration for DigitalNZ heritage records.

use crate::models::Record;
use polars::prelude::*;

/// Extension trait to convert lists of Records directly to Polars DataFrames.
pub trait IntoDataFrame {
    /// Convert search records into a Polars DataFrame.
    fn into_dataframe(self) -> Result<DataFrame, PolarsError>;
}

impl IntoDataFrame for Vec<Record> {
    fn into_dataframe(self) -> Result<DataFrame, PolarsError> {
        let mut ids = Vec::with_capacity(self.len());
        let mut titles = Vec::with_capacity(self.len());
        let mut descriptions = Vec::with_capacity(self.len());
        let mut content_partners = Vec::with_capacity(self.len());
        let mut categories = Vec::with_capacity(self.len());

        for rec in self {
            ids.push(rec.id);
            titles.push(rec.title);
            descriptions.push(rec.description.unwrap_or_default());
            content_partners.push(rec.content_partner.map(|v| v.join(", ")).unwrap_or_default());
            categories.push(rec.category.map(|v| v.join(", ")).unwrap_or_default());
        }

        let id_series = Series::new("id", ids);
        let title_series = Series::new("title", titles);
        let desc_series = Series::new("description", descriptions);
        let partner_series = Series::new("content_partner", content_partners);
        let category_series = Series::new("category", categories);

        DataFrame::new(vec![
            id_series,
            title_series,
            desc_series,
            partner_series,
            category_series,
        ])
    }
}
