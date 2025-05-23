# BOM_forge
An Altium BOM parser that finds you available parts from LCSC and Mouser.
# Rationale and Mission.
Automating Tedious BOM prep in embedded systems teams and removing (time)costly BOM mistakes.

Using the same footprint+symbol for every resistor and capacitor and renaming(or changing the comment) it for all values is nice for workflow and consistence of footprints. Quite often, the specific part number or source of the symbol and footprint you place does not matter. For a specific resistor, you might just want any resistor with the resistance to a specific tolerance package, and while compiling the BOM you're looking for the best deal with the best avaliability. 

A counter argument is the annoying side effects of using the same symbols. Taking a 100nF capacitor and renaming it to use as a 4.7uF, 10uF and 22uF, might produce several capacitors grouped with a "100nF, 4.7uF, 10uF, 22uF" name. Face Palm emoji.

This script aims to fix annoying side effects as well as automate the process of manually finding LCSC links and numbers for each part. 

# Roadmap
## Core
- [x] Data Structures for Resistors, Capacitors and ICs.
- [x] Parsing Altium BOMs.
- [ ] HTTP + HTML scraping for LCSC.
- [ ] Mouser Fallback
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
