use std::collections::HashMap;
use crate::storage::load_page;
use crate::parser::extract_text;

pub struct SearchEngine {
    index: HashMap<String, Vec<String>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            index: HashMap::new(),
        }
    }
}
