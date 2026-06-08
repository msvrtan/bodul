use super::sitemap_configuration::SitemapConfiguration;

#[derive(Debug, Clone)]
pub struct AcquisitionProfile {
    pub capabilities: Vec<CapabilityType>,
    pub sitemap_configuration: Option<SitemapConfiguration>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityType {
    SitemapCatalogDiscovery,
    SitemapProductDiscovery,
    WebsiteCatalogDiscovery,
    CatalogTraversal,
    ProductRetrieval,
    PriceRefresh,
    AvailabilityRefresh,
}

impl AcquisitionProfile {
    pub fn new(capabilities: Vec<CapabilityType>) -> Self {
        Self {
            capabilities,
            sitemap_configuration: None,
        }
    }

    pub fn with_sitemap_configuration(
        mut self,
        config: SitemapConfiguration,
    ) -> Self {
        self.sitemap_configuration = Some(config);
        self
    }

    pub fn has_capability(&self, capability: CapabilityType) -> bool {
        self.capabilities.contains(&capability)
    }
}
