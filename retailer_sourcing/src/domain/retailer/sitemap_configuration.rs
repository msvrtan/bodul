#[derive(Debug, Clone)]
pub struct SitemapConfiguration {
    pub sitemap_urls: Vec<String>,
    pub catalog_page_patterns: Vec<String>,
    pub product_page_patterns: Vec<String>,
}

impl SitemapConfiguration {
    pub fn new(
        sitemap_urls: Vec<String>,
        catalog_page_patterns: Vec<String>,
        product_page_patterns: Vec<String>,
    ) -> Self {
        Self {
            sitemap_urls,
            catalog_page_patterns,
            product_page_patterns,
        }
    }
}
