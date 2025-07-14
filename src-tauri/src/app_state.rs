use crate::config::AppConfig;
use crate::files_in_dirs::model::FilesInDirs;
use crate::state_manager::JsonState;
use crate::task_queue::task_queue::ThreadSafeEventQueue;
use crate::thumb_gen::thumb_store::ThumbnailStore;

// Application state to hold todos and file path
pub struct AppState {
    pub event_queue: ThreadSafeEventQueue,
    pub thumbnail_store: ThumbnailStore,
    pub app_config: JsonState<AppConfig>,
    pub files_in_dirs: JsonState<FilesInDirs>,
}
