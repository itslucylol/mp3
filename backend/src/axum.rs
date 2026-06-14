use axum::Router;
use axum::routing::{ get, post };
use tower_http::services::ServeDir;
use std::sync::{Arc, Mutex};

use crate::routes;

#[tokio::main]
pub async fn main() {

    let state = crate::routes::queue::AppState {
        now_playing: Arc::new(Mutex::new(None)),
    };

    let api_router = Router::new()
        .route("/info", get(routes::info::main))
        .route("/library", get(routes::library::main))
        .route("/play", post(routes::queue::play_track))
        .route("/nowplaying", get(routes::queue::get_now_playing))
        .with_state(state);


    let serve_dir = ServeDir::new("public");
    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}