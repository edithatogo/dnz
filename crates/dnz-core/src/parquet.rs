//! Optional Parquet export backed by Arrow.

use crate::export::{atomic_replace, temporary_path};
use crate::models::Record;
use arrow_array::{ArrayRef, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;
use polars_core::error::{PolarsError, PolarsResult};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

/// Write the stable tabular record projection as Parquet.
pub fn write_records_parquet(path: impl AsRef<Path>, records: Vec<Record>) -> PolarsResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    }
    let ids: Vec<String> = records.iter().map(|record| record.id.clone()).collect();
    let titles: Vec<String> = records.iter().map(|record| record.title.clone()).collect();
    let descriptions: Vec<String> = records
        .iter()
        .map(|record| record.description.clone().unwrap_or_default())
        .collect();
    let content_partners: Vec<String> = records
        .iter()
        .map(|record| {
            record
                .content_partner
                .as_ref()
                .map(|values| values.join(", "))
                .unwrap_or_default()
        })
        .collect();
    let categories: Vec<String> = records
        .iter()
        .map(|record| {
            record
                .category
                .as_ref()
                .map(|values| values.join(", "))
                .unwrap_or_default()
        })
        .collect();

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("title", DataType::Utf8, false),
        Field::new("description", DataType::Utf8, false),
        Field::new("content_partner", DataType::Utf8, false),
        Field::new("category", DataType::Utf8, false),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(ids)) as ArrayRef,
            Arc::new(StringArray::from(titles)) as ArrayRef,
            Arc::new(StringArray::from(descriptions)) as ArrayRef,
            Arc::new(StringArray::from(content_partners)) as ArrayRef,
            Arc::new(StringArray::from(categories)) as ArrayRef,
        ],
    )
    .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    let temporary = temporary_path(path);
    let file = File::create(&temporary)
        .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    let mut writer = ArrowWriter::try_new(file, schema, None)
        .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    writer
        .write(&batch)
        .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    writer
        .close()
        .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    atomic_replace(&temporary, path)
        .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    Ok(())
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
        let file = File::open(&path).unwrap();
        let reader = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(file)
            .unwrap()
            .build()
            .unwrap();
        let batches: Vec<_> = reader.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(
            batches.iter().map(|batch| batch.num_rows()).sum::<usize>(),
            1
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn parquet_publish_replaces_existing_destination() {
        let path = std::env::temp_dir().join(format!(
            "dnz-records-replace-{}-{}.parquet",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(&path, b"stale output").unwrap();
        write_records_parquet(
            &path,
            vec![Record {
                id: "2".into(),
                title: "Tui".into(),
                ..Record::default()
            }],
        )
        .unwrap();
        let file = File::open(&path).unwrap();
        let reader = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(file)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            reader.map(|batch| batch.unwrap().num_rows()).sum::<usize>(),
            1
        );
        let _ = std::fs::remove_file(path);
    }
}
