#[derive(Debug)]
pub struct ResourceResponse {
    pub status_code: u16,
    pub content: Vec<u8>,
    pub headers: std::collections::HashMap<String, String>,
}

impl ResourceResponse {
    pub fn new(status_code: u16, content: Vec<u8>) -> Self {
        Self {
            status_code,
            content,
            headers: std::collections::HashMap::new(),
        }
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }

    pub fn content_as_string(&self) -> Result<String, String> {
        String::from_utf8(self.content.clone())
            .map_err(|e| format!("Failed to decode content as UTF-8: {}", e))
    }
}
