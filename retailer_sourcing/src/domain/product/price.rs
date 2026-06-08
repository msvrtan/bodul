#[derive(Debug, Clone, Copy)]
pub struct Price {
    pub amount: f64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    EUR,
    USD,
}

impl Price {
    pub fn new(amount: f64, currency: Currency) -> Self {
        Self { amount, currency }
    }
}
