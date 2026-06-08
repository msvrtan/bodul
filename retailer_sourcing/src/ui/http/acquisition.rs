use poem::http::StatusCode;
use poem::web::Json;
use poem::{Route, handler, post};
use serde::Deserialize;

#[derive(Deserialize)]
struct SitemapDiscoveryRequest {
    retailer_codes: Vec<String>,
}

#[handler]
async fn sitemap_discovery(
    Json(_req): Json<SitemapDiscoveryRequest>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

#[handler]
async fn website_catalog_discovery() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

#[handler]
async fn catalog_traversal() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

#[handler]
async fn product_retrieval() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

#[handler]
async fn price_refresh() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

#[handler]
async fn availability_refresh() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub fn routes() -> Route {
    Route::new()
        .at("/sitemap-discovery", post(sitemap_discovery))
        .at(
            "/website-catalog-discovery",
            post(website_catalog_discovery),
        )
        .at("/catalog-traversal", post(catalog_traversal))
        .at("/product-retrieval", post(product_retrieval))
        .at("/price-refresh", post(price_refresh))
        .at("/availability-refresh", post(availability_refresh))
}
