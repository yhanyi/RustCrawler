use std::collections::VecDeque;
use std::sync::{ Arc, Mutex };
use std::thread;
use reqwest;
use crate::parser::extract_links;
use crate::storage::store_page;

pub struct Crawler {
    to_visit: Arc<Mutex<VecDeque<String>>>,
    visited: Arc<Mutex<Vec<String>>>,
    max_depth: usize,
}

impl Crawler {
    pub fn new(seed_urls: Vec<String>, max_depth: usize) -> Self {
        let to_visit = Arc::new(Mutex::new(VecDeque::from(seed_urls)));
        let visited = Arc::new(Mutex::new(Vec::new()));
        Crawler { to_visit, visited, max_depth }
    }

    pub fn crawl(&self) {
        let mut handles = vec![];

        for _ in 0..4 {
            // Use 4 threads for crawling
            let to_visit = Arc::clone(&self.to_visit);
            let visited = Arc::clone(&self.visited);
            let max_depth = self.max_depth;

            let handle = thread::spawn(move || {
                while
                    let Some(url) = ({
                        let mut queue = to_visit.lock().unwrap();
                        queue.pop_front()
                    })
                {
                    if visited.lock().unwrap().contains(&url) {
                        continue;
                    }

                    match fetch_and_process_url(&url, max_depth) {
                        Ok(new_urls) => {
                            let mut queue = to_visit.lock().unwrap();
                            queue.extend(new_urls);
                            visited.lock().unwrap().push(url);
                        }
                        Err(e) => eprintln!("Error processing {}: {}", url, e),
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn fetch_and_process_url(
    url: &str,
    depth: usize
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fetch_url(url)?;
    store_page(url, &content)?;
    let new_urls = if depth > 0 { extract_links(&content, url)? } else { vec![] };
    Ok(new_urls)
}

fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(url)?.text()
}
