use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::fs;
use std::path::Path;
use serde::Deserialize;
use std::sync::{Arc, Mutex};

use crate::LIBRARY_ROOT;

#[derive(Clone)]
pub struct AppState {
    // Stores the relative path to the currently playing file (e.g., "Music/song.mp3")
    pub now_playing: Arc<Mutex<Option<String>>>,
}

#[derive(Deserialize)]
pub struct PlayRequest {
    pub file: String,
}

pub async fn play_track(
    State(state): State<AppState>,
    Json(payload): Json<PlayRequest>,
) -> StatusCode {
    let mut now_playing = state.now_playing.lock().unwrap();
    *now_playing = Some(payload.file);
    StatusCode::OK
}

pub async fn get_now_playing(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let now_playing = state.now_playing.lock().unwrap();
    
    // Check if a track is actually playing
    let file_subpath = match &*now_playing {
        Some(path) => path,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Sanitize and build the safe absolute path to prevent traversal attacks
    let mut target_path = std::path::PathBuf::from(LIBRARY_ROOT);
    for component in Path::new(file_subpath).components() {
        if let std::path::Component::Normal(c) = component {
            target_path.push(c);
        }
    }

    // Read the file bytes and return them
    let file_bytes = fs::read(&target_path).map_err(|_| StatusCode::NOT_FOUND)?;
    
    // Axum automatically converts `Vec<u8>` into a standard binary body response
    Ok(file_bytes)
}