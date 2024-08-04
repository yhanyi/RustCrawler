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

    pub fn search(&self, query: &str) -> Vec<String> {
        let query_words: Vec<String> = query.split_whitespace().map(str::to_lowercase).collect();
        let mut results: HashMap<String, usize> = HashMap::new();
        for word in query_words {
            if let Some(urls) = self.index.get(&word) {
                for url in urls {
                    *results.entry(url.to_string()).or_default() += 1;
                }
            }
        }
        let mut sorted_results: Vec<_> = results.into_iter().collect();
        sorted_results.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_results
            .into_iter()
            .map(|(url, _)| url)
            .collect()
    }
}
