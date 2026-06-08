use poem::{Route, Server, listener::TcpListener};

pub mod acquisition;
mod health;

pub fn routes() -> Route {
    Route::new()
        .nest("/health", health::routes())
        .nest("/acquisition", acquisition::routes())
}

pub async fn serve(addr: &str) -> Result<(), std::io::Error> {
    Server::new(TcpListener::bind(addr)).run(routes()).await
}
