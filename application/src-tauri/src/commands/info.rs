use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
pub struct InfoPayload {
    version: String,
    platform: String,
    #[serde(rename = "storageUsed")]
    storage_used: String,
    #[serde(rename = "storageMax")]
    storage_max: String,
}

#[tauri::command]
pub fn info() -> Result<InfoPayload, String> {
    let output = Command::new("df")
        .args(["-h", "/"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Failed to execute df command".into());
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

    Ok(InfoPayload {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: std::env::consts::ARCH.to_string(),
        storage_used: used,
        storage_max: max,
    })
}