use crate::{
    app_state::AppState,
    files_in_dirs::file::File,
    serde_utils::deserialize_vec_skip_errors,
    task_queue::task::{GenerateThumbTask, Task},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DirWithFiles {
    pub path: String,

    #[serde(default, deserialize_with = "deserialize_vec_skip_errors")]
    pub files: Vec<File>,
}

impl DirWithFiles {
    pub fn new(dir: &String, state: &tauri::State<AppState>) -> Result<Self, String> {
        let dir_clone = dir.clone();
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory '{}': {}", dir_clone, e))?;

        let mut dir_with_files = DirWithFiles {
            path: dir_clone.clone(),
            files: Vec::new(),
        };

        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.is_file() {
                let metadata = path.metadata().unwrap();

                let file = File {
                    id: nanoid::nanoid!(),
                    name: path.file_name().unwrap().to_string_lossy().into_owned(),
                    size: metadata.len(),
                    thumbs: Vec::new(), // Thumbnails will be generated later
                };

                dir_with_files.files.push(file.clone());
            }
        }

        let total: u32 = dir_with_files
            .files
            .len()
            .try_into()
            .expect("Too many files in directory");

        for (i, file) in dir_with_files.files.iter().enumerate() {
            state
                .event_queue
                .enqueue(Task::GenerateThumb(GenerateThumbTask {
                    dir: dir_clone.clone(),
                    id: file.id.clone(),
                    i: i as u32,
                    total,
                }));
        }

        Ok(dir_with_files)
    }
}

// TODO: move loading files from FilesInDirs to DirWithFiles::new

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FilesInDirs {
    pub dirs: Vec<DirWithFiles>,
}

impl FilesInDirs {
    /// Finds a file by directory path and file id
    pub fn find_file(&self, dir: &str, id: &str) -> Option<&File> {
        self.dirs
            .iter()
            .find(|d| d.path == dir)?
            .files
            .iter()
            .find(|f| f.id == id)
    }

    /// Finds a mutable file by directory path and file id
    pub fn find_file_mut(&mut self, dir: &str, id: &str) -> Option<&mut File> {
        self.dirs
            .iter_mut()
            .find(|d| d.path == dir)?
            .files
            .iter_mut()
            .find(|f| f.id == id)
    }

    // TODO: Add functions add_dir, rm_dir, rescan_dir, rescan_all
    pub fn add_dir(&mut self, dir: String, state: &tauri::State<AppState>) -> Result<(), String> {
        if self.dirs.iter().any(|d| d.path == dir) {
            return Err(format!("Directory '{}' already exists", dir));
        }

        let dir_with_files = DirWithFiles::new(&dir, &state)?;
        self.dirs.push(dir_with_files);
        Ok(())
    }

    pub fn remove_dir(&mut self, dir: &String) -> Result<(), String> {
        if let Some(pos) = self.dirs.iter().position(|d| &d.path == dir) {
            self.dirs.remove(pos);
            // TODO queue removal of files in the dir.
            Ok(())
        } else {
            Err(format!("Directory '{}' not found", dir))
        }
    }

    // TODO: implement this smarter, so it doesn't re-read all files
    pub fn rescan_dir(
        &mut self,
        dir: &String,
        state: &tauri::State<AppState>,
    ) -> Result<(), String> {
        if let Some(dir_with_files) = self.dirs.iter_mut().find(|d| &d.path == dir) {
            *dir_with_files = DirWithFiles::new(dir, state)?;
            Ok(())
        } else {
            Err(format!("Directory '{}' not found", dir))
        }
    }
}
