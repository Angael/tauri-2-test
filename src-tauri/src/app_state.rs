use std::path::PathBuf;
use std::sync::Mutex;

use crate::config::AppConfig;
use crate::files_in_dirs::model::FilesInDirs;
use crate::state_manager::JsonState;

// Application state to hold todos and file path
pub struct AppState {
    pub app_config: JsonState<AppConfig>,
    pub files_in_dirs: JsonState<FilesInDirs>,
}
