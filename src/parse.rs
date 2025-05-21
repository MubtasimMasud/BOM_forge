use anyhow::{Context, Result, anyhow, bail};
use csv::{ReaderBuilder, StringRecord};
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

pub fn find_ambigious_names(rows: Vec<PreprocessedBOMEntry>) -> Vec<String> {
    let mut ambigious_elements = Vec::new();

    for row in rows {
        if row.name.clone().split(',').count() > 1 {
            ambigious_elements.push(row.name);
        }
    }
    ambigious_elements
}
// Seeing what values are designated to which designators to address ambigious Name Coulms
// returns Hashmap<Designator, Value> from the CPL file
pub fn parse_cpl(path: &str) -> Result<HashMap<String, String>> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

    //Finding the header within the CPL file cuz it has an annoying title block
    let mut header: Option<StringRecord> = None;
    for result in rdr.records() {
        let record = result?;
        if record.get(0).map(|s| s.trim()) == Some("Designator") {
            header = Some(record);
            break;
        }
    }
    let header = header.ok_or_else(|| anyhow!("bruh where is the header in this csv file"))?;

    let header_map: HashMap<&str, usize> = header
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim(), i))
        .collect();

    // adjust if your “comment” column is named differently
    let designator_index = *header_map
        .get("Designator")
        .ok_or_else(|| anyhow!("invalid index"))?;
    let comment_index = *header_map
        .get("Comment")
        .ok_or_else(|| anyhow!("invalid index"))?;

    let mut parts_map: HashMap<String, String> = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let designator = record
            .get(designator_index)
            .map(str::trim)
            .ok_or_else(|| anyhow!("could not get designator"))?
            .to_string();
        let comment = record
            .get(comment_index)
            .map(str::trim)
            .ok_or_else(|| anyhow!("could not get comment"))?
            .to_string();

        parts_map.insert(designator, comment);
    }
    Ok(parts_map)
}

fn get_replacements_for_ambigious(
    cpl_parts_map: HashMap<String, String>,
    bom_entries: Vec<PreprocessedBOMEntry>,
) -> Vec<PreprocessedBOMEntry> {
    todo!()
}
