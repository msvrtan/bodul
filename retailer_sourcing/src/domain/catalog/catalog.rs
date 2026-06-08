#[derive(Debug, Clone)]
pub struct CatalogId(pub String);

#[derive(Debug, Clone)]
pub struct Catalog {
    pub id: CatalogId,
    pub name: String,
    pub url: String,
}

impl Catalog {
    pub fn new(id: CatalogId, name: String, url: String) -> Self {
        Self { id, name, url }
    }
}
