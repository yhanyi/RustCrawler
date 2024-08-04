use std::fs;
use std::io;
use std::path::Path;

pub fn store_page(url: &str, content: &str) -> io::Result<()> {
    let file_name = url_to_file_name(url);
    let path = Path::new("pages").join(file_name);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, content)
}

fn url_to_file_name(url: &str) -> String {
    url.replace("://", "_").replace('/', "_").replace(':', "_") + ".html"
}

pub fn load_page(url: &str) -> io::Result<String> {
    let file_name = url_to_file_name(url);
    let path = Path::new("pages").join(file_name);
    fs::read_to_string(path)
}
