use super::CatalogNode;

#[derive(Debug, Clone)]
pub struct CatalogTree {
    pub root: CatalogNode,
}

impl CatalogTree {
    pub fn new(root: CatalogNode) -> Self {
        Self { root }
    }
}
