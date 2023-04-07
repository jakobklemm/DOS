//! # Translator

use std::collections::HashMap;
use std::fs;

pub struct Dictionary {
    pub list: HashMap<String, String>,
}

impl Dictionary {
    pub fn load() -> Self {
        let f = fs::read_to_string("table.json").unwrap();
        let m: HashMap<String, String> = serde_json::from_str(&f).unwrap();
        Self { list: m }
    }
}
