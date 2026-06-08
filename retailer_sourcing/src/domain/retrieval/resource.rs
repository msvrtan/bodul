#[derive(Debug, Clone)]
pub enum Resource {
    Sitemap(String),
    Homepage(String),
    CatalogPage(String),
    ProductPage(String),
    ApiResponse(String),
}

impl Resource {
    pub fn url(&self) -> &str {
        match self {
            Resource::Sitemap(url) => url,
            Resource::Homepage(url) => url,
            Resource::CatalogPage(url) => url,
            Resource::ProductPage(url) => url,
            Resource::ApiResponse(url) => url,
        }
    }
}
