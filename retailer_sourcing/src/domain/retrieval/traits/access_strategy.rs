use crate::domain::retrieval::{ResourceRequest, ResourceResponse};

pub trait AccessStrategy: Send + Sync {
    fn execute(
        &self,
        request: ResourceRequest,
    ) -> Result<ResourceResponse, String>;
    fn name(&self) -> &str;
}
