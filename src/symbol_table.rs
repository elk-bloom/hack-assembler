use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &str) -> Option<&u16> {
        self.table.get(symbol)
    }
}
