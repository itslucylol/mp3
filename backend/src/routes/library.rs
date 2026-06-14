use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{ Path, PathBuf, Component };

const LIBRARY_ROOT: &str = "./library";
const REQUIRED_FOLDERS: [&str; 3] = ["Music", "Photos", "Videos"];

fn ensure_root_structure() {
    for folder in REQUIRED_FOLDERS.iter() {
        let folder_path = Path::new(LIBRARY_ROOT).join(folder); // e.g., "./library/Music"
        let _ = fs::create_dir_all(&folder_path); // create_dir_all safely does nothing if the folder already exists
    }
}

#[derive(Deserialize)]
pub struct LibraryQuery {
    path: Option<String>,
}

pub async fn main(Query(params): Query<LibraryQuery>) -> Result<Json<Value>, StatusCode> {

    ensure_root_structure();

    //---   Build Target Path (Within Library Folder)   ---//
    let mut target_path = PathBuf::from(LIBRARY_ROOT);
    if let Some(sub_path) = &params.path {
        let user_path = Path::new(sub_path); // Create a Path from the input string
        for component in user_path.components() { // Iterate through the user string to sanitize it
            match component {
                // Ignore absolute roots (like /) or prefixes (like C:) to prevent absolute paths
                Component::RootDir | Component::Prefix(_) => continue,
                
                // Ignore parents (..) so users can't climb out of the folder
                Component::ParentDir => continue,
                
                // Normal files/directories and current dir (.) are perfectly safe
                Component::Normal(c) => target_path.push(c),
                Component::CurDir => continue,
            }
        }
    }

    let entries = fs::read_dir(&target_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut files = Vec::new();
    let mut directories = Vec::new();

    for entry in entries {
        // If an entry fails to read, skip it or handle the error
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    directories.push(file_name);
                } else {
                    files.push(file_name);
                }
            }
        }
    }

    return Ok(Json(json!({
        "directories": directories,
        "files": files
    })));
}