pub mod components {
    enum Package {
        Passive0402,
        Passive0603,
        Passive0805,
        Passive1206,
        LQFP64IC,
        QFN40IC,
        THT,
    }
    struct Resistor {
        resistence: u32,
        package: Package,
    }
    struct Capacitor {
        capacitence: u32,
        package: Package,
    }
    struct IC {
        part_number: String,
        package: Package,
    }
}
