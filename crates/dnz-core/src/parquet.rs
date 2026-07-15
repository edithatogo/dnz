//! Optional Parquet export backed by Polars.

use crate::dataframe::IntoDataFrame;
use crate::models::Record;
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

/// Write the stable tabular record projection as Parquet.
pub fn write_records_parquet(path: impl AsRef<Path>, records: Vec<Record>) -> PolarsResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    }
    let mut frame = records.into_dataframe()?;
    let file =
        File::create(path).map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    ParquetWriter::new(file).finish(&mut frame).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parquet_writer_emits_a_nonempty_artifact() {
        let path = std::env::temp_dir().join(format!(
            "dnz-records-{}-{}.parquet",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        write_records_parquet(
            &path,
            vec![Record {
                id: "1".into(),
                title: "Kauri".into(),
                ..Record::default()
            }],
        )
        .unwrap();
        assert!(std::fs::metadata(&path).unwrap().len() > 0);
        let _ = std::fs::remove_file(path);
    }
}
