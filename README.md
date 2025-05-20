# BOM_forge
An Altium BOM parser that finds you available parts from LCSC and Mouser.
# Rationale
Automating Tedious BOM prep in embedded systems teams and removing (time)costly BOM mistakes.
# Roadmap
## Core
- [ ] Rust Project setup: Clap, anyhow + thiserror, tokio.
- [ ] Data Structures for Resistors, Capacitors and ICs.
- [ ] Parsing Altium and KiCAD BOMs.
- [ ] HTTP + HTML scraping for LCSC.
- [ ] Mouser API Implementation.
- [ ] Final BOM Building.
## After Core
- [ ] Front end through egui or iced.

# Documentation
## Parsing Altium
There are a few patterns to account for. 
- `100nF, 1uF, 4.7uF, 10uF, 8pF` in name columns of the cap of the same Footprint
- `150k, 28k, 10k, 6.2k, 120R` need to parse units appropriately.
See ambigious cases

### Thoughts
- We should ignore part names for passives
- Checking `R`, `C`, `D` & `FB` etc in designators for types. Should be pretty fast and reliable
- We should ignore any duplicates for rows of the same component types and values. 
### Ambigious cases
- I can use the CPL files to distinguish ambiguity in BOM rows with duplicate resistor and capacitor values.
- to check for an ambigious case I can check the string for a comma. Then for each csv I can make Vecs of relevant designators. 
