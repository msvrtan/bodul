use super::Resource;

#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub resource: Resource,
    pub headers: Option<std::collections::HashMap<String, String>>,
}

impl ResourceRequest {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            headers: None,
        }
    }

    pub fn with_headers(
        mut self,
        headers: std::collections::HashMap<String, String>,
    ) -> Self {
        self.headers = Some(headers);
        self
    }
}
