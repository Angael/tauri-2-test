use std::path::PathBuf;
use std::sync::Mutex;

use crate::files_in_dirs::model::FilesInDirs;
use crate::todo::Todo;

// Application state to hold todos and file path
pub struct AppState {
    // Paths
    pub data_file_path: PathBuf, // Deprecated
    pub files_in_dirs_path: PathBuf,

    // State
    pub files_in_dirs: Mutex<FilesInDirs>,
    pub todos: Mutex<Vec<Todo>>, // Deprecated
    pub next_id: Mutex<u32>,     // Deprecated
}
