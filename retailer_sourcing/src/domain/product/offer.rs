#[derive(Debug, Clone)]
pub struct Offer {
    pub product_id: String,
    pub retailer_id: String,
    pub price: f64,
    pub currency: String,
}

impl Offer {
    pub fn new(
        product_id: String,
        retailer_id: String,
        price: f64,
        currency: String,
    ) -> Self {
        Self {
            product_id,
            retailer_id,
            price,
            currency,
        }
    }
}
