mod crawler;
mod parser;
mod storage;
mod search;
mod web_interface;

use crawler::Crawler;
use search::SearchEngine;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed_urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://doc.rust-lang.org".to_string()
    ];
    let crawler = Crawler::new(seed_urls, 2);
    crawler.crawl();
    let mut search_engine = SearchEngine::new();
    for url in &["https://www.rust-lang.org", "https://doc.rust-lang.org"] {
        search_engine.index_page(url)?;
    }
    web_interface::launch(search_engine)?;

    Ok(())
}
