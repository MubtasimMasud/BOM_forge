use std::collections::HashMap;

enum ComponentType {
    Resistor(u32),  // resistance stored in ohms
    Capacitor(u32), // capacitance stored in pF
    IC(String),     // string containing part number
    Other(String),  // string containing part number
}

struct Component {
    part_type: ComponentType,
    package: String,
    designators: Vec<String>,
}

struct BOM {
    pcb_name: String,
    bom: HashMap<Component, u32>, // component and Qty
}

impl BOM {
    pub fn new(self, name: String) -> BOM {
        BOM {
            pcb_name: name,
            bom: HashMap::new(),
        }
    }
}
