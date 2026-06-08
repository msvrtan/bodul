use super::{Retailer, RetailerId};

pub trait RetailerRepository: Send + Sync {
    fn find_by_code(
        &self,
        retailer_id: &RetailerId,
    ) -> Result<Option<Retailer>, String>;
}
