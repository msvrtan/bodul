#[derive(Debug, Clone)]
pub struct CatalogPage {
    pub url: String,
    pub content: String,
}

impl CatalogPage {
    pub fn new(url: String, content: String) -> Self {
        Self { url, content }
    }
}
