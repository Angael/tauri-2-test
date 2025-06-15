use std::path::PathBuf;
use std::sync::Mutex;

use crate::config::AppConfig;
use crate::files_in_dirs::model::FilesInDirs;

// Application state to hold todos and file path
pub struct AppState {
    // Paths
    pub files_in_dirs_path: PathBuf,

    // State
    pub app_config: Mutex<AppConfig>,
    pub files_in_dirs: Mutex<FilesInDirs>,
}
