use anyhow;
use csv::ReaderBuilder;
use thiserror;

#[derive(Debug)]
struct PreprocessedBOMEntry {
    name: String,
    description: String,
    mpn: String,
    designators: Vec<String>,
}

fn read_preprocessed_BOM(path: &str) -> Vec<PreprocessedBOMEntry> {
    todo!()
}
