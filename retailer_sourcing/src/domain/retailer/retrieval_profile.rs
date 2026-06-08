#[derive(Debug, Clone)]
pub struct RetrievalProfile {
    pub access_strategy: AccessStrategyType,
    pub proxy_pool: Option<String>,
    pub browser_profile: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessStrategyType {
    Http,
    Proxy,
    Browser,
    BrowserWithProxy,
    Agent,
}

impl RetrievalProfile {
    pub fn new(
        access_strategy: AccessStrategyType,
        proxy_pool: Option<String>,
        browser_profile: Option<String>,
    ) -> Self {
        Self {
            access_strategy,
            proxy_pool,
            browser_profile,
        }
    }
}
