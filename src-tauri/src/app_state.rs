use std::path::PathBuf;
use std::sync::Mutex;

use crate::saved_folders::SavedFolder;
use crate::todo::Todo;

// Application state to hold todos and file path
pub struct AppState {
    // Paths
    pub data_file_path: PathBuf,
    pub saved_folders_path: PathBuf,

    // State
    pub saved_folders: Mutex<Vec<SavedFolder>>,
    pub todos: Mutex<Vec<Todo>>,
    pub next_id: Mutex<u32>, // To generate unique IDs
}
