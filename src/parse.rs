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

#[derive(Debug, Clone)]
pub struct PreprocessedBOM {
    pub bom: Vec<PreprocessedBOMEntry>,
}

impl PreprocessedBOM {
    pub fn from_csv(path: &str) -> Result<Self> {
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
                name: Self::extract_field("Name", &header_map, &record),
                description: Self::extract_field("Description", &header_map, &record),
                mpn: Self::extract_field("Part Number", &header_map, &record),
                designators: Self::extract_field("Designator", &header_map, &record)
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
            };
            entries.push(entry);
        }
        Ok(PreprocessedBOM { bom: entries })
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

    pub fn fix_ambigious_rows(&mut self, cpl_parts: &CplData) -> Result<()> {
        let ambigous_names = Self::find_ambigious_names(&self);
        let fixed_entries = self.get_replacements_for_ambigious(&cpl_parts)?;

        self.bom
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
            .collect::<Vec<PreprocessedBOMEntry>>();
        for entry in fixed_entries {
            self.bom.push(entry);
        }
        Ok(())
    }

    fn get_replacements_for_ambigious(
        &self,
        cpl_parts: &CplData,
    ) -> Result<Vec<PreprocessedBOMEntry>> {
        let cases = Self::find_ambigious_names(self);

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
                    designators: Self::find_designators_for_name(&name, cpl_parts),
                };
                fixed_rows.push(entry);
            }
        }
        Ok(fixed_rows)
    }

    pub fn find_designators_for_name(part_name: &str, cpl_parts: &CplData) -> Vec<String> {
        let designators: Vec<String> = cpl_parts
            .parts_map
            .clone()
            .into_iter()
            .filter(|part| part.value == part_name)
            .map(|part| part.designator)
            .collect();
        designators
    }

    pub fn find_ambigious_names(&self) -> Vec<String> {
        let mut ambigious_elements = Vec::new();

        for row in &self.bom {
            if row.name.clone().split(',').count() > 1 {
                ambigious_elements.push(row.name.clone());
            }
        }
        ambigious_elements
    }
}

#[derive(Debug, Clone)]
pub struct CplDataEntry {
    designator: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct CplData {
    parts_map: Vec<CplDataEntry>,
}

impl CplData {
    // Seeing what values are designated to which designators to address ambigious Name Coulms
    // returns Hashmap<Value, Designator> from the CPL file
    pub fn from_csv(path: &str) -> Result<Self> {
        // Seeing what values are designated to which designators to address ambigious Name Coulms
        // returns Hashmap<Value, Designator> from the CPL file
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

        let mut parts_map: Vec<CplDataEntry> = Vec::new();
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

            parts_map.push(CplDataEntry {
                designator: designator,
                value: comment,
            });
        }
        Ok(CplData { parts_map })
    }
}
