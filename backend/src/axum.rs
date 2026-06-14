use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::routes;

#[tokio::main]
pub async fn main() {
    let api_router = Router::new()
        .route("/info", get(routes::info::main))
        .route("/library", get(routes::library::main));


    let serve_dir = ServeDir::new("public");
    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}