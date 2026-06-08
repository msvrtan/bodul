use std::sync::Arc;

use crate::domain::acquisition::DiscoveredSitemap;
use crate::domain::acquisition::commands::discover_sitemap::DiscoverSitemap;
use crate::domain::retailer::sitemap_configuration::SitemapConfiguration;
use crate::domain::retailer::{RetailerId, RetailerRepository};
use crate::domain::retrieval::traits::ResourceAcquirer;
use crate::domain::retrieval::{Resource, ResourceRequest, ResourceResponse};

/// Intermediate classification used while bucketing URLs from a sitemap.
/// Catalog wins over Product when a URL matches both.
enum UrlKind {
    Catalog,
    Product,
    /// Any URL that does not match catalog or product patterns.
    Misc,
}

/// Errors that can occur during sitemap discovery.
#[derive(Debug)]
pub enum SitemapDiscoveryError {
    /// No retailer found for the given retailer code.
    RetailerNotFound(RetailerId),
    /// The retailer exists but has no sitemap configuration.
    NoSitemapConfiguration,
    /// An HTTP or I/O error occurred while fetching a sitemap.
    FetchFailed(String),
    /// The sitemap response could not be parsed.
    ParseFailed(String),
}

/// Discovers and classifies a retailer's sitemap URLs in five steps:
/// fetch index → extract child URLs → filter to root locale → fetch children → classify pages.
///
/// Driven by the retailer's `SitemapConfiguration`, which defines the sitemap index
/// URLs to fetch and which URL patterns map to each page type.
pub struct SitemapDiscoveryHandler {
    repo: Arc<dyn RetailerRepository>,
    client: Arc<dyn ResourceAcquirer>,
}

impl SitemapDiscoveryHandler {
    #[rustfmt::skip]
    pub fn new(repo: Arc<dyn RetailerRepository>, client: Arc<dyn ResourceAcquirer>) -> Self {
        Self { repo, client }
    }
}

impl SitemapDiscoveryHandler {
    /// Runs the full sitemap discovery pipeline:
    /// load config → fetch index → extract child URLs → filter to root locale
    /// → fetch children → extract page URLs → classify into catalog/product/misc.
    #[rustfmt::skip]
    pub async fn handle(&self, cmd: DiscoverSitemap) -> Result<DiscoveredSitemap, SitemapDiscoveryError> {
        let config = self.retailer_sitemap_config(&cmd.retailer_code)?;
        let index_responses = self.fetch_sitemap_indexes(&config)?;
        let child_sitemap_urls = Self::extract_child_sitemap_urls(index_responses)?;
        let root_sitemap_urls = Self::filter_locale_sitemaps(child_sitemap_urls);
        let child_responses = self.fetch_child_sitemaps(root_sitemap_urls)?;
        let page_urls = Self::extract_page_urls(child_responses)?;
        let discovered_sitemap = Self::classify_urls(page_urls, &config);

        Ok(discovered_sitemap)
    }

    /// Loads the retailer and extracts its sitemap configuration.
    fn retailer_sitemap_config(
        &self,
        retailer_code: &RetailerId,
    ) -> Result<SitemapConfiguration, SitemapDiscoveryError> {
        let retailer = self
            .repo
            .find_by_code(retailer_code)
            .map_err(SitemapDiscoveryError::FetchFailed)?
            .ok_or_else(|| {
                SitemapDiscoveryError::RetailerNotFound(retailer_code.clone())
            })?;

        let sitemap_configuration = retailer
            .acquisition_profile
            .sitemap_configuration
            .ok_or(SitemapDiscoveryError::NoSitemapConfiguration)?;

        Ok(sitemap_configuration)
    }

    /// Fetches the sitemap index files (`sitemap.xml`) listed in the config.
    fn fetch_sitemap_indexes(
        &self,
        config: &SitemapConfiguration,
    ) -> Result<Vec<ResourceResponse>, SitemapDiscoveryError> {
        let mut responses = Vec::new();
        for sitemap_url in &config.sitemap_urls {
            let request =
                ResourceRequest::new(Resource::Sitemap(sitemap_url.clone()));

            let response = self
                .client
                .acquire(request)
                .map_err(SitemapDiscoveryError::FetchFailed)?;

            responses.push(response);
        }
        Ok(responses)
    }

    /// Parses sitemap index responses and returns the child sitemap URLs they contain.
    fn extract_child_sitemap_urls(
        indexes: Vec<ResourceResponse>,
    ) -> Result<Vec<String>, SitemapDiscoveryError> {
        let mut urls = Vec::new();
        for index in indexes {
            urls.extend(Self::parse_sitemap_index(index)?);
        }
        Ok(urls)
    }

    /// Parses a single sitemap index XML response and returns the child sitemap URLs.
    fn parse_sitemap_index(
        response: ResourceResponse,
    ) -> Result<Vec<String>, SitemapDiscoveryError> {
        let _content = response
            .content_as_string()
            .map_err(SitemapDiscoveryError::ParseFailed)?;

        todo!("parse sitemap index XML and extract child sitemap URLs")
    }

    /// Filters out locale-prefixed child sitemaps, keeping only root locale entries.
    ///
    /// Shopify sitemap indexes include one set of child sitemaps per locale:
    /// - root:   `/sitemap_products_1.xml`    ← kept
    /// - locale: `/de/sitemap_products_1.xml` ← dropped
    ///
    /// A URL is considered locale-prefixed when its path contains more than one
    /// non-empty segment (e.g. `/de/sitemap_...` has segments ["de", "sitemap_..."]).
    fn filter_locale_sitemaps(urls: Vec<String>) -> Vec<String> {
        urls.into_iter()
            .filter(|url| !Self::is_locale_sitemap(url))
            .collect()
    }

    /// Returns true if the URL points to a locale-prefixed child sitemap.
    fn is_locale_sitemap(url: &str) -> bool {
        let path = url
            .find("://")
            .map(|i| &url[i + 3..])
            .and_then(|s| s.find('/').map(|i| &s[i..]))
            .map(|s| s.split('?').next().unwrap_or(s))
            .unwrap_or("");

        let segment_count = path.split('/').filter(|s| !s.is_empty()).count();

        segment_count > 1
    }

    /// Fetches each child sitemap file.
    fn fetch_child_sitemaps(
        &self,
        urls: Vec<String>,
    ) -> Result<Vec<ResourceResponse>, SitemapDiscoveryError> {
        let mut responses = Vec::new();
        for url in urls {
            let request = ResourceRequest::new(Resource::Sitemap(url));

            let response = self
                .client
                .acquire(request)
                .map_err(SitemapDiscoveryError::FetchFailed)?;

            responses.push(response);
        }
        Ok(responses)
    }

    /// Parses all child sitemap responses and aggregates their page URLs into a flat list.
    fn extract_page_urls(
        sitemaps: Vec<ResourceResponse>,
    ) -> Result<Vec<String>, SitemapDiscoveryError> {
        let mut all_urls = Vec::new();
        for sitemap in sitemaps {
            all_urls.extend(Self::parse_sitemap(sitemap)?);
        }
        Ok(all_urls)
    }

    /// Parses a single child sitemap XML response and returns the page URLs it contains.
    fn parse_sitemap(
        response: ResourceResponse,
    ) -> Result<Vec<String>, SitemapDiscoveryError> {
        let _content = response
            .content_as_string()
            .map_err(SitemapDiscoveryError::ParseFailed)?;

        todo!("parse sitemap XML and extract page URLs")
    }

    /// Sorts URLs into catalog, product, and misc buckets based on config patterns.
    fn classify_urls(
        urls: Vec<String>,
        config: &SitemapConfiguration,
    ) -> DiscoveredSitemap {
        let mut catalog_pages = Vec::new();
        let mut product_pages = Vec::new();
        let mut misc_pages = Vec::new();

        for url in urls {
            match Self::url_kind(&url, config) {
                UrlKind::Catalog => catalog_pages.push(url),
                UrlKind::Product => product_pages.push(url),
                UrlKind::Misc => misc_pages.push(url),
            }
        }

        let discovered_sitemap = DiscoveredSitemap {
            catalog_pages,
            product_pages,
            misc_pages,
        };
        discovered_sitemap
    }

    /// Determines the kind of a single URL by matching it against config patterns.
    /// Catalog takes priority — if a URL matches both catalog and product patterns,
    /// it is classified as Catalog.
    fn url_kind(url: &str, config: &SitemapConfiguration) -> UrlKind {
        let is_catalog = config
            .catalog_page_patterns
            .iter()
            .any(|pattern| url.contains(pattern.as_str()));

        if is_catalog {
            return UrlKind::Catalog;
        }

        let is_product = config
            .product_page_patterns
            .iter()
            .any(|pattern| url.contains(pattern.as_str()));

        if is_product {
            return UrlKind::Product;
        }

        UrlKind::Misc
    }
}
