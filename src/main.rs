mod crawler;
mod parser;
mod storage;
mod search;
mod web_interface;

use crawler::Crawler;
use search::SearchEngine;
use std::sync::Arc;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
}
