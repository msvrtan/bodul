use crate::domain::retrieval::{ResourceRequest, ResourceResponse};

pub trait ResourceAcquirer: Send + Sync {
    fn acquire(
        &self,
        request: ResourceRequest,
    ) -> Result<ResourceResponse, String>;
}
