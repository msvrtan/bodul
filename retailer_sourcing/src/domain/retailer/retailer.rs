use super::{AcquisitionProfile, RetrievalProfile};

#[derive(Debug, Clone)]
pub struct RetailerId(pub String);

#[derive(Debug, Clone)]
pub struct Retailer {
    pub id: RetailerId,
    pub name: String,
    pub acquisition_profile: AcquisitionProfile,
    pub retrieval_profile: RetrievalProfile,
}

impl Retailer {
    pub fn new(
        id: RetailerId,
        name: String,
        acquisition_profile: AcquisitionProfile,
        retrieval_profile: RetrievalProfile,
    ) -> Self {
        Self {
            id,
            name,
            acquisition_profile,
            retrieval_profile,
        }
    }
}
