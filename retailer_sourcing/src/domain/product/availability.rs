#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Availability {
    InStock,
    OutOfStock,
    PreOrder,
    Unknown,
}

impl Availability {
    pub fn is_available(&self) -> bool {
        matches!(self, Availability::InStock | Availability::PreOrder)
    }
}
