use poem::{Route, get, handler};

#[handler]
async fn check() -> &'static str {
    "ok"
}

pub fn routes() -> Route {
    Route::new().at("/", get(check))
}
