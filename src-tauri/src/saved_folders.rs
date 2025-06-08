use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::commands::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedFolder {
    pub path: String,
}

#[tauri::command]
pub fn get_saved_folders(state: tauri::State<AppState>) -> Result<Vec<SavedFolder>, String> {
    println!("Retrieving saved folders from state");
    let state = state
        .saved_folders
        .lock()
        .map_err(|e| format!("Failed to lock saved folders: {}", e))?;
    println!("Saved folders: {:?}", state);

    Ok(state.clone())
}

#[tauri::command]
pub fn save_folders(
    saved_folders: Vec<SavedFolder>,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    let saved_folders_str = serde_json::to_string(&saved_folders)
        .map_err(|e| format!("Failed to serialize saved folders: {}", e))?;

    std::fs::write(&state.saved_folders_path, saved_folders_str)
        .expect("Failed to write saved folders to file");
    Ok(())
}

// Helper function to load todos from disk
pub fn load_folders_from_disk(path: &PathBuf) -> Vec<SavedFolder> {
    if path.exists() {
        let data = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|err| {
            eprintln!(
                "Failed to parse saved folders from {}: {}, starting with empty list.",
                path.display(),
                err
            );
            Vec::new()
        })
    } else {
        Vec::new()
    }
}
