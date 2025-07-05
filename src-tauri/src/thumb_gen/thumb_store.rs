use tauri::Manager;

pub struct ThumbnailStore {
    pub dir: std::path::PathBuf,
}

impl ThumbnailStore {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        let dir = app_handle
            .path()
            .app_cache_dir()
            .expect("Error getting cache dir")
            .join("files");

        std::fs::create_dir_all(&dir).expect("Failed to create thumbnail directory");

        Self { dir }
    }

    pub fn get_file_dir(&self, file_id: &str) -> std::path::PathBuf {
        self.dir.join(file_id)
    }

    pub fn get_thumbnail_path(&self, file_id: &str, thumb_name: &str) -> std::path::PathBuf {
        self.get_file_dir(file_id).join(thumb_name)
    }
}
