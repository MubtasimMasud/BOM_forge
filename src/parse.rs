use anyhow::{Context, Result, anyhow, bail};
use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use thiserror;

#[derive(Debug, Clone)]
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
// returns Hashmap<Value, Designator> from the CPL file
pub fn parse_cpl(path: &str) -> Result<Vec<(String, String)>> {
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

    let mut parts_map: Vec<(String, String)> = Vec::new();
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

        parts_map.push((comment, designator));
    }
    Ok(parts_map)
}

pub fn fix_ambigious_rows(
    cpl_parts: Vec<(String, String)>,
    bom_entries: Vec<PreprocessedBOMEntry>,
) -> Result<Vec<PreprocessedBOMEntry>> {
    let ambigous_names = find_ambigious_names(bom_entries.clone());
    let fixed_entries = get_replacements_for_ambigious(cpl_parts.clone(), bom_entries.clone())?;

    let mut filtered_bom: Vec<PreprocessedBOMEntry> = bom_entries
        .clone()
        .into_iter()
        .filter(|entry| {
            for name in ambigous_names.iter() {
                if &entry.name == name {
                    return false;
                }
            }
            true
        })
        .collect();
    for entry in fixed_entries {
        filtered_bom.push(entry);
    }
    Ok(filtered_bom)
}

fn get_replacements_for_ambigious(
    cpl_parts: Vec<(String, String)>,
    bom_entries: Vec<PreprocessedBOMEntry>,
) -> Result<Vec<PreprocessedBOMEntry>> {
    let cases = find_ambigious_names(bom_entries);

    let mut fixed_rows: Vec<PreprocessedBOMEntry> = Vec::new();
    for case in cases {
        // find designators for each value(name)
        let names: Vec<String> = case
            .clone()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        for name in names {
            let entry = PreprocessedBOMEntry {
                name: name.clone(),
                description: String::new(),
                mpn: String::new(),
                designators: find_designators_for_name(&name, &cpl_parts),
            };
            fixed_rows.push(entry);
        }
    }
    Ok(fixed_rows)
}

fn find_designators_for_name(part_name: &str, cpl_parts: &Vec<(String, String)>) -> Vec<String> {
    let designators: Vec<String> = cpl_parts
        .clone()
        .into_iter()
        .filter(|(name, _)| name == part_name)
        .map(|(_, designator)| designator)
        .collect();
    designators
}
