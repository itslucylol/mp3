use std::process::Command;
use axum::{http::StatusCode, Json};

pub async fn main() -> Result<Json<serde_json::Value>, StatusCode> {
    let output = Command::new("df")
        .args(["-h", "/"])
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // If the command exits with a non-zero code, return an HTTP 500 error
    if !output.status.success() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    let (used, max) = if lines.len() >= 2 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() >= 3 {
            (parts[2].to_string(), parts[1].to_string())
        } else {
            ("Unknown".into(), "Unknown".into())
        }
    } else {
        ("Unknown".into(), "Unknown".into())
    };

    // Return successful information.
    return Ok(Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION").to_string(),
        "platform": std::env::consts::ARCH.to_string(),
        "storageUsed": used,
        "storageMax": max,
    })));
}