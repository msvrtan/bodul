use super::{Availability, Price};

#[derive(Debug, Clone)]
pub struct ProductId(pub String);

#[derive(Debug, Clone)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub url: String,
    pub price: Price,
    pub availability: Availability,
}

impl Product {
    pub fn new(
        id: ProductId,
        name: String,
        url: String,
        price: Price,
        availability: Availability,
    ) -> Self {
        Self {
            id,
            name,
            url,
            price,
            availability,
        }
    }
}
