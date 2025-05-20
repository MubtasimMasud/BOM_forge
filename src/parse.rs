use anyhow::Result;
use csv::ReaderBuilder;
use std::collections::HashMap;
use thiserror;

#[derive(Debug)]
pub struct PreprocessedBOMEntry {
    name: String,
    description: String,
    mpn: String,
    designators: Vec<String>,
}

pub fn read_preprocessed_bom(path: &str) -> Result<Vec<PreprocessedBOMEntry>> {
    let mut rdr = ReaderBuilder::new().flexible(true).from_path(path)?;
    let headers = rdr.headers()?.clone();

    let header_map: HashMap<_, _> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim(), i))
        .collect();

    let mut entries = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let entry = PreprocessedBOMEntry {
            name: extract_field("Name", &header_map, &record),
            description: extract_field("Description", &header_map, &record),
            mpn: extract_field("Part Number", &header_map, &record),
            designators: extract_field("Designator", &header_map, &record)
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        };
        entries.push(entry);
    }
    Ok(entries)
}

//helper function for above
// returns empty string if nothing found
fn extract_field(
    field_name: &str,
    headers: &HashMap<&str, usize>,
    record: &csv::StringRecord,
) -> String {
    if let Some(&index) = headers.get(&field_name) {
        if let Some(val) = record.get(index) {
            return val.trim().to_string();
        }
    }
    String::new()
}
