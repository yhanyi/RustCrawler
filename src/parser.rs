use scraper::{ Html, Selector };
use url::Url;

pub fn extract_links(content: &str, base_url: &str) -> Result<Vec<String>, url::ParseError> {
    let document = Html::parse_document(content);
    let selector = Selector::parse("a").unwrap();
    let base_url = Url::parse(base_url)?;

    let mut links = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(url) = base_url.join(href) {
                links.push(url.into());
            }
        }
    }
    Ok(links)
}

pub fn extract_text(content: &str) -> String {
    let document = Html::parse_document(content);
    document.tree
        .values()
        .filter_map(|n| n.as_text())
        .collect()
}
