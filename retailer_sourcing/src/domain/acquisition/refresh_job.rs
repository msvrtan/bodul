use crate::domain::retailer::RetailerId;

#[derive(Debug, Clone)]
pub struct RefreshJobId(pub String);

#[derive(Debug, Clone)]
pub enum RefreshJobType {
    DailyRefresh,
    RetailerRefresh,
    PriceRefresh,
    AvailabilityRefresh,
}

#[derive(Debug, Clone)]
pub struct RefreshJob {
    pub id: RefreshJobId,
    pub retailer_id: Option<RetailerId>,
    pub job_type: RefreshJobType,
}

impl RefreshJob {
    pub fn new(id: RefreshJobId, job_type: RefreshJobType) -> Self {
        Self {
            id,
            retailer_id: None,
            job_type,
        }
    }

    pub fn for_retailer(
        id: RefreshJobId,
        retailer_id: RetailerId,
        job_type: RefreshJobType,
    ) -> Self {
        Self {
            id,
            retailer_id: Some(retailer_id),
            job_type,
        }
    }
}
