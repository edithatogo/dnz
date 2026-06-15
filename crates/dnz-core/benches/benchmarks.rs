//! Microbenchmarks for the DigitalNZ client library using Divan.

use dnz_core::models::Record;
use dnz_core::IntoDataFrame;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_record_to_dataframe() {
    let mut records = Vec::with_capacity(100);
    for i in 0..100 {
        records.push(Record {
            id: format!("rec_{}", i),
            title: format!("Record Title {}", i),
            description: Some(
                "A standard record description used during benchmark suites.".to_string(),
            ),
            collection: Some(vec!["GLAM-Data".to_string()]),
            content_partner: Some(vec!["Partner Institutional System".to_string()]),
            category: Some(vec!["Documents".to_string()]),
            ..Default::default()
        });
    }

    let _df = records.into_dataframe().unwrap();
}

#[divan::bench]
fn bench_serde_record_parsing() {
    let raw_json = r#"{
        "id": "999",
        "title": "Historical Auckland Photograph",
        "description": "Black and white historical photograph from 1890.",
        "collection": ["Auckland-Historical"],
        "content_partner": ["Auckland Council"],
        "category": ["Images"],
        "date": ["1890-01-01"]
    }"#;

    let _parsed: Record = serde_json::from_str(divan::black_box(raw_json)).unwrap();
}
