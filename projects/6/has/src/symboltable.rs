use std::collections::HashMap;

pub struct SymbolTable {
    m: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn init() -> SymbolTable {
        let known_symbols: HashMap<String, u16> = HashMap::from([
            ("SP".to_string(), 0x0),
            ("LCL".to_string(), 0x1),
            ("ARG".to_string(), 0x2),
            ("THIS".to_string(), 0x3),
            ("THAT".to_string(), 0x4),
            ("R0".to_string(), 0x0),
            ("R1".to_string(), 0x1),
            ("R2".to_string(), 0x2),
            ("R3".to_string(), 0x3),
            ("R4".to_string(), 0x4),
            ("R5".to_string(), 0x5),
            ("R6".to_string(), 0x6),
            ("R7".to_string(), 0x7),
            ("R8".to_string(), 0x8),
            ("R9".to_string(), 0x9),
            ("R10".to_string(), 0xa),
            ("R11".to_string(), 0xb),
            ("R12".to_string(), 0xc),
            ("R13".to_string(), 0xd),
            ("R14".to_string(), 0xe),
            ("R15".to_string(), 0xf),
            ("SCREEN".to_string(), 0x4000),
            ("KBD".to_string(), 0x6000),
        ]);

        SymbolTable { m: known_symbols }
    }

    pub fn add_new_label(&mut self, label: String, instr_num: u16) {
        self.m.entry(label).or_insert(instr_num + 1);
    }
}
