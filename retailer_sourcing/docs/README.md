# retailer_sourcing — Documentation

## Retailer Data Acquisition Strategies

There are three main strategies available for scheduling data acquisition from retailers:

1. **Web Scraping** — Directly crawling and scraping product data from retailer websites
2. **Sitemap Scraping** — Extracting product data from retailer site maps
3. **API Integration** — Using retailer APIs for direct data access

Different retailers can be scheduled with different strategies or combinations of strategies to optimize data quality and collection efficiency.

## Minisforum Stores

| Code          | Store                | Sitemap                                  | Additional Locales          |
|---------------|----------------------|------------------------------------------|-----------------------------|
| MinisForumAu  | au.minisforum.com    | https://au.minisforum.com/sitemap.xml    | —                           |
| MinisForumCa  | ca.minisforum.com    | https://ca.minisforum.com/sitemap.xml    | —                           |
| MinisForumEu  | minisforumpc.eu      | https://minisforumpc.eu/sitemap.xml      | `/de`                       |
| MinisForumFr  | minisforumpc.fr      | https://minisforumpc.fr/sitemap.xml      | `/en`                       |
| MinisForumUk  | www.minisforum.uk    | https://www.minisforum.uk/sitemap.xml    | —                           |
| MinisForumUs  | store.minisforum.com | https://store.minisforum.com/sitemap.xml | `/es` · `/en-ca` · `/en-os` |

### Sitemap Fetching

All Minisforum stores run on Shopify, which means `sitemap.xml` is always a sitemap index — not a flat list of URLs. The fetch process works in two steps:

1. Fetch `sitemap.xml` — parse the index and extract the child sitemap URLs
2. Fetch each child sitemap one by one — parse and extract the page URLs from each

```
sitemap.xml (index)
    ↓ extract child sitemap URLs
sitemap_products_1.xml
sitemap_pages_1.xml
sitemap_collections_1.xml
sitemap_blogs_1.xml
    ↓ extract page URLs from each
[all page URLs]
```

For stores with additional locales (e.g. MinisForumEu with `/de`), the index contains a full set of child sitemaps per locale. Each locale's child sitemaps are fetched the same way.

**MVP scope:** Only the default (root) locale is fetched. Additional locale sitemaps are skipped.

## Daily Update Workflow

Each day, the system performs a full acquisition for all active retailers. The approach varies by retailer:

- **Sitemap-based retailers** — Go through the retailer's sitemap and load all product links for discovery
- **Web scraping retailers** — Crawl and discover all product links
- **API-based retailers** — Fetch complete product data (approach TBD)

### Incremental Updates

In addition to full acquisition with discovery, the system can also perform targeted updates by looking for products already in the system and refreshing them without full product discovery.

## Pricing and Availability Updates

Pricing and availability updates are scheduled independently per retailer, as different retailers require different update frequencies:

- Some retailers: updates multiple times per day
- Some retailers: updates a few times per week

### Availability Data Sources

The sources for availability data vary by retailer based on their data structure:

- **Catalog listings** — Some retailers expose availability information directly in their product catalog/listing pages
- **Product detail pages** — Some retailers require fetching individual product data to determine availability information

## Domain Organization

The system is organized into distinct domains, each with specific responsibilities:

```
Retailer
├── Configuration
├── AcquisitionCapabilities
└── RetrievalProfile

Acquisition
├── Workflows
├── Jobs
├── Resource Graph
└── Scheduling

Retrieval
├── Resource Acquisition
├── Access Strategies
├── Proxy Management
├── Browser Management
└── Agent Execution

Catalog
├── Catalog Trees
├── Catalog Pages
└── Catalog Discovery

Product
├── Product Discovery
├── Product Retrieval
├── Price Refresh
└── Availability Refresh
```

### Domain Responsibilities

- **Retailer** — Configuration and capabilities for a specific retailer (what acquisition and retrieval strategies are available)
- **Acquisition** — Orchestrates WHAT to fetch through workflows and jobs, with scheduling logic
- **Retrieval** — Handles HOW to fetch (strategies, resource acquisition, proxies, browsers, agents)
- **Catalog** — Manages catalog/listing pages and catalog discovery
- **Product** — Handles product-level operations (discovery, retrieval, pricing updates, availability updates)

## Retrieval, Acquisition, and Scraping Architecture

Once you introduce proxies, browsers, agents, anti-bot bypasses, APIs, scraping services, etc., there is another important separation:

**What data do we want?** versus **How do we obtain it?**

Most scraping systems eventually become unmaintainable because those concerns get mixed together.

### Layer 1: Acquisition Workflow

Business intent.

```
Discover Catalogs
Discover Products
Fetch Product Pages
Refresh Prices
Refresh Availability
```

Example workflow:

```
StartRetailerRefresh
    ↓
Discover Products
    ↓
Fetch Products
    ↓
Extract Products
```

This layer should not know:
- proxy providers
- browsers
- Playwright
- residential IPs
- AI agents
- CAPTCHA solving

### Layer 2: Resource Acquisition

This layer answers: **How do I obtain this URL?**

Capabilities:
```
HttpDownload
BrowserDownload
ApiRequest
AgentNavigation
```

Interface:

```rust
trait ResourceAcquirer {
    fn acquire(
        &self,
        request: ResourceRequest,
    ) -> ResourceResponse;
}
```

### Layer 3: Access Strategy

This layer decides: **Which mechanism should be used?**

Examples:
```
SimpleHttpStrategy
ProxyStrategy
BrowserStrategy
BrowserWithProxyStrategy
AgentStrategy
```

```rust
trait AccessStrategy {
    fn execute(
        &self,
        request: ResourceRequest,
    ) -> ResourceResponse;
}
```

### Layer 4: Infrastructure

Actual tools:
```
BrightData
Oxylabs
Playwright
Chrome
Firefox
OpenAI Operator
ScrapingBee
Custom Proxy Pool
```

### Example Retailer Configurations

Retailer A (Sitemap + Browser):
```yaml
retailer: retailer-a

workflow:
  - SitemapProductDiscovery
  - ProductRetrieval

access:
  strategy: BrowserWithProxy
```

Retailer B (Simple HTTP):
```yaml
retailer: retailer-b

workflow:
  - SitemapProductDiscovery

access:
  strategy: Http
```

Retailer C (Agent-based):
```yaml
retailer: retailer-c

workflow:
  - WebsiteCatalogDiscovery
  - CatalogTraversal

access:
  strategy: Agent
```

### Resource-Centric Model

A model that scales very well is treating everything as a resource:

```
Resource
├── Sitemap
├── Homepage
├── CatalogPage
├── ProductPage
├── ApiResponse
```

Workflow example:

```
Need Product URLs
    ↓
Acquire Sitemap
    ↓
Parse Sitemap
    ↓
Product URLs
```

The acquisition engine doesn't care whether the sitemap came from HTTP, Browser, Proxy, or Agent.

### Retrieval Profile

A retrieval profile encapsulates how to access resources:

```rust
struct RetrievalProfile {
    access_strategy: AccessStrategy,
    proxy_pool: ProxyPool,
    browser_profile: BrowserProfile,
    rate_limits: RateLimits,
}
```

Example profiles:
```
FastApiProfile
CheapHttpProfile
ResidentialBrowserProfile
AntiBotProfile
AgentProfile
```

Retailers reference profiles:
```
Lidl → CheapHttpProfile
Nike → ResidentialBrowserProfile
Amazon → AntiBotProfile
```

### Core Principle

```
Acquisition decides WHAT to fetch.

Retrieval decides HOW to fetch it.

Infrastructure performs the actual fetch.
```

Once you enforce that boundary, adding a new proxy provider, browser technology, or AI agent usually becomes a Retrieval concern and doesn't force changes to Product Discovery, Catalog Traversal, or Daily Refresh workflows. That's the separation that keeps these systems maintainable as they grow.

## Code Organization

### Domain Layer

```
domain/
├── retailer/
│   ├── retailer.rs
│   ├── acquisition_profile.rs
│   └── retrieval_profile.rs
│
├── acquisition/
│   ├── refresh_job.rs
│   ├── workflow.rs
│   ├── workflow_step.rs
│   ├── capabilities/
│   │   ├── sitemap_catalog_discovery.rs
│   │   ├── sitemap_product_discovery.rs
│   │   ├── website_catalog_discovery.rs
│   │   ├── catalog_traversal.rs
│   │   ├── product_retrieval.rs
│   │   ├── price_refresh.rs
│   │   └── availability_refresh.rs
│   │
│   ├── commands/
│   │   ├── start_daily_refresh.rs
│   │   ├── start_retailer_refresh.rs
│   │   ├── discover_catalogs.rs
│   │   ├── discover_products.rs
│   │   └── fetch_products.rs
│   │
│   └── events/
│       ├── daily_refresh_started.rs
│       ├── retailer_refresh_started.rs
│       ├── catalogs_discovered.rs
│       ├── products_discovered.rs
│       └── products_fetched.rs
│
├── retrieval/
│   ├── resource.rs
│   ├── resource_request.rs
│   ├── resource_response.rs
│   │
│   ├── strategies/
│   │   ├── http_strategy.rs
│   │   ├── proxy_strategy.rs
│   │   ├── browser_strategy.rs
│   │   └── agent_strategy.rs
│   │
│   └── traits/
│       ├── resource_acquirer.rs
│       └── access_strategy.rs
│
├── catalog/
│   ├── catalog.rs
│   ├── catalog_node.rs
│   ├── catalog_tree.rs
│   └── catalog_page.rs
│
└── product/
    ├── product.rs
    ├── offer.rs
    ├── price.rs
    └── availability.rs
```

### Application Layer

```
app/
├── commands/
│   ├── start_daily_refresh_handler.rs
│   ├── start_retailer_refresh_handler.rs
│   ├── discover_catalogs_handler.rs
│   ├── discover_products_handler.rs
│   └── fetch_products_handler.rs
│
├── workflows/
│   ├── refresh_coordinator.rs
│   ├── retailer_refresh_workflow.rs
│   └── workflow_builder.rs
│
├── projectors/
│   ├── catalog_projector.rs
│   └── product_projector.rs
│
└── services/
    ├── capability_executor.rs
    └── retrieval_router.rs
```

### UI Layer

```
ui/
└── http/
    ├── acquisition.rs   # POST /acquisition/{capability}
    └── health.rs        # GET /health
```

HTTP endpoints are Poem handlers grouped by domain. Each handler maps to one acquisition capability:

```
POST /acquisition/sitemap-discovery
POST /acquisition/website-catalog-discovery
POST /acquisition/catalog-traversal
POST /acquisition/product-retrieval
POST /acquisition/price-refresh
POST /acquisition/availability-refresh
```

### Infrastructure Layer

```
infra/
├── postgres/
│
├── inbox/
│
├── outbox/
│
├── scheduler/
│   └── cron_scheduler.rs
│
├── retrieval/
│   ├── reqwest_client.rs
│   ├── playwright_client.rs
│   ├── proxy_pool.rs
│   └── agent_client.rs
│
└── retailers/
    ├── kaufland/
    │   ├── parsers.rs
    │   ├── extractors.rs
    │   └── mappings.rs
    │
    ├── lidl/
    └── spar/
```

## Core Traits

### Workflow

```rust
pub trait WorkflowStep {
    fn execute(
        &self,
        ctx: &WorkflowContext,
    ) -> Result<WorkflowResult>;
}
```

### Capability

```rust
pub trait Capability {
    fn execute(
        &self,
        ctx: CapabilityContext,
    ) -> Result<CapabilityResult>;
}
```

Examples: `SitemapCatalogDiscovery`, `WebsiteCatalogDiscovery`, `CatalogTraversal`, `ProductRetrieval`, `PriceRefresh`

### Retrieval

```rust
pub trait ResourceAcquirer {
    fn acquire(
        &self,
        request: ResourceRequest,
    ) -> Result<ResourceResponse>;
}
```

### Access Strategy

```rust
pub trait AccessStrategy {
    fn execute(
        &self,
        request: ResourceRequest,
    ) -> Result<ResourceResponse>;
}
```

Implementations: `HttpStrategy`, `ProxyStrategy`, `BrowserStrategy`, `BrowserWithProxyStrategy`, `AgentStrategy`

## Retailer Aggregate

```rust
pub struct Retailer {
    pub id: RetailerId,
    pub name: String,
    pub acquisition_profile: AcquisitionProfile,
    pub retrieval_profile: RetrievalProfile,
}
```

### Acquisition Profile

```rust
pub struct AcquisitionProfile {
    pub capabilities: Vec<CapabilityType>,
}
```

Example:
```rust
AcquisitionProfile {
    capabilities: vec![
        CapabilityType::SitemapCatalogDiscovery,
        CapabilityType::WebsiteCatalogDiscovery,
        CapabilityType::CatalogTraversal,
        CapabilityType::ProductRetrieval,
    ]
}
```

### Retrieval Profile

```rust
pub struct RetrievalProfile {
    pub access_strategy: AccessStrategyType,
    pub proxy_pool: Option<ProxyPoolId>,
    pub browser_profile: Option<BrowserProfileId>,
}
```

## Refresh Coordinator

This is the central entry point orchestrating the entire acquisition flow:

```rust
pub struct RefreshCoordinator {
    retailer_repository: Arc<dyn RetailerRepository>,
    command_bus: Arc<dyn CommandBus>,
}

impl RefreshCoordinator {
    pub fn start_daily_refresh(
        &self,
        cmd: StartDailyRefresh,
    );

    pub fn start_retailer_refresh(
        &self,
        cmd: StartRetailerRefresh,
    );

    fn build_workflow(
        &self,
        retailer: &Retailer,
    ) -> Workflow;
}
```

## Entry Points

Everything converges on the `RefreshCoordinator`:

```
HTTP / Scheduler / CLI / Inbox
            ↓
    StartRetailerRefresh
            ↓
    RefreshCoordinator
            ↓
Retailer → AcquisitionProfile → WorkflowBuilder → Workflow
            ↓
        Workflow Steps:
            ↓
SitemapCatalogDiscovery → CatalogTraversal → ProductRetrieval → ProductExtraction
```

## Architectural Rule

**Retailers configure capabilities, Workflows orchestrate capabilities, Retrieval fetches resources, and Infrastructure provides the concrete tools.**

This separation ensures:
- New proxy providers → Retrieval layer change
- New browser engines → Retrieval layer change
- New AI agents → Retrieval layer change
- New retailers → Configuration layer change
- New workflows → Acquisition layer change

This design will survive hundreds of retailers, dozens of proxy providers, multiple browser engines, and future acquisition mechanisms without forcing cascading changes across the system.

