#[derive(Debug, Clone)]
pub struct CatalogNode {
    pub name: String,
    pub url: String,
    pub parent: Option<Box<CatalogNode>>,
    pub children: Vec<CatalogNode>,
}

impl CatalogNode {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            parent: None,
            children: Vec::new(),
        }
    }
}
