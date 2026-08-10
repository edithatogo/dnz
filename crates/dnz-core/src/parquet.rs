//! Optional Parquet export backed by Arrow.

use crate::export::{atomic_replace, find_coordinates, temporary_path};
use crate::models::Record;
use arrow_array::{ArrayRef, BinaryArray, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;
use polars_core::error::{PolarsError, PolarsResult};
use serde_json::json;
use std::collections::HashMap;
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

/// Write records with verified point locations as a GeoParquet 1.1 dataset.
///
/// Records without finite WGS84 coordinates are omitted. Geometry is encoded
/// as little-endian WKB points, and the required `geo` schema metadata names
/// the geometry column and its encoding.
pub fn write_records_geoparquet(path: impl AsRef<Path>, records: Vec<Record>) -> PolarsResult<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| PolarsError::ComputeError(error.to_string().into()))?;
    }

    let rows: Vec<_> = records
        .iter()
        .filter_map(|record| {
            record
                .locations
                .as_ref()
                .and_then(find_coordinates)
                .map(|coordinates| (record, coordinates))
        })
        .collect();
    let ids: Vec<String> = rows.iter().map(|(record, _)| record.id.clone()).collect();
    let titles: Vec<String> = rows
        .iter()
        .map(|(record, _)| record.title.clone())
        .collect();
    let source_urls: Vec<Option<String>> = rows
        .iter()
        .map(|(record, _)| record.source_url.clone())
        .collect();
    let geometries: Vec<Vec<u8>> = rows
        .iter()
        .map(|(_, (longitude, latitude))| point_wkb(*longitude, *latitude))
        .collect();

    let geo_metadata = json!({
        "version": "1.1.0",
        "primary_column": "geometry",
        "columns": {
            "geometry": {
                "encoding": "WKB",
                "geometry_types": ["Point"],
                "crs": null
            }
        }
    })
    .to_string();
    let schema = Arc::new(Schema::new_with_metadata(
        vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("title", DataType::Utf8, false),
            Field::new("source_url", DataType::Utf8, true),
            Field::new("geometry", DataType::Binary, false),
        ],
        HashMap::from([("geo".to_string(), geo_metadata)]),
    ));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(ids)) as ArrayRef,
            Arc::new(StringArray::from(titles)) as ArrayRef,
            Arc::new(StringArray::from(source_urls)) as ArrayRef,
            Arc::new(BinaryArray::from_iter_values(geometries)) as ArrayRef,
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

fn point_wkb(longitude: f64, latitude: f64) -> Vec<u8> {
    let mut wkb = Vec::with_capacity(21);
    wkb.push(1);
    wkb.extend_from_slice(&1_u32.to_le_bytes());
    wkb.extend_from_slice(&longitude.to_le_bytes());
    wkb.extend_from_slice(&latitude.to_le_bytes());
    wkb
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_array::cast::AsArray;

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

    #[test]
    fn geoparquet_emits_wkb_points_and_geo_metadata() {
        let path = std::env::temp_dir().join(format!(
            "dnz-records-{}-{}.geoparquet",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let valid = Record {
            id: "1".into(),
            title: "Kauri".into(),
            locations: Some(serde_json::json!({"longitude": 174.76, "latitude": -36.85})),
            ..Record::default()
        };
        let invalid = Record {
            id: "2".into(),
            locations: Some(serde_json::json!({"longitude": 181.0, "latitude": -36.85})),
            ..Record::default()
        };
        write_records_geoparquet(&path, vec![valid, invalid]).unwrap();
        let file = File::open(&path).unwrap();
        let reader =
            parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
        let geo = reader.schema().metadata().get("geo").unwrap();
        let metadata: serde_json::Value = serde_json::from_str(geo).unwrap();
        assert_eq!(metadata["version"], "1.1.0");
        assert_eq!(metadata["columns"]["geometry"]["encoding"], "WKB");
        let batches: Vec<_> = reader
            .build()
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(batches[0].num_rows(), 1);
        assert_eq!(batches[0].column(3).as_binary::<i32>().value(0).len(), 21);
        let _ = std::fs::remove_file(path);
    }
}
