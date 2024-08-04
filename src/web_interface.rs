// web_interface.rs
use rocket::{ self, get, post, routes };
use rocket::form::Form;
use rocket::FromForm;
use rocket::response::content::RawHtml;
use crate::search::SearchEngine;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <form action="/search" method="post">
            <input type="text" name="query">
            <input type="submit" value="Search">
        </form>
    "#
    )
}

#[derive(FromForm)]
struct SearchQuery {
    query: String,
}

#[post("/search", data = "<search_query>")]
fn search(
    search_query: Form<SearchQuery>,
    search_engine: &rocket::State<SearchEngine>
) -> RawHtml<String> {
    let results = search_engine.search(&search_query.query);
    let results_html = results
        .iter()
        .map(|url| format!("<li><a href=\"{}\">{}</a></li>", url, url))
        .collect::<Vec<_>>()
        .join("\n");

    RawHtml(
        format!(
            r#"
        <h1>Search Results for "{}"</h1>
        <ul>
            {}
        </ul>
        <a href="/">Back to Search</a>
        "#,
            search_query.query,
            results_html
        )
    )
}

pub fn launch(search_engine: SearchEngine) -> Result<(), Box<dyn std::error::Error>> {
    rocket::build().manage(search_engine).mount("/", routes![index, search]).launch();
    Ok(())
}
