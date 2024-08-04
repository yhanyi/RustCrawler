use std::collections::VecDeque;
use std::sync::{ Arc, Mutex };
use std::thread;
use reqwest;
use url::Url;
use crate::parser::extract_links;
use crate::storage::store_page;

pub struct Crawler {
    to_visit: Arc<Mutex<VecDeque<String>>>,
    visited: Arc<Mutex<VecDeque<String>>>,
    max_depth: usize,
}
