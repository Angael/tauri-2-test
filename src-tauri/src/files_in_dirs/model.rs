use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    name: String,
    size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DirWithFiles {
    pub path: String,
    pub files: Vec<File>,
}

impl DirWithFiles {
    pub fn new(dir: &String) -> Result<Self, String> {
        let dir_clone = dir.clone();
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory '{}': {}", dir_clone, e))?;

        let mut dir_with_files = DirWithFiles {
            path: dir_clone,
            files: Vec::new(),
        };

        for entry in entries {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();

            if path.is_file() {
                let metadata = path.metadata().unwrap();
                dir_with_files.files.push(File {
                    name: path.file_name().unwrap().to_string_lossy().into_owned(),
                    size: metadata.len(),
                });
            }
        }

        Ok(dir_with_files)
    }
}

// TODO: move loading files from FilesInDirs to DirWithFiles::new

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesInDirs {
    pub dirs: Vec<DirWithFiles>,
}

impl FilesInDirs {
    // Unused
    pub fn new(dirs: Vec<String>) -> Self {
        let mut files_in_dirs = FilesInDirs { dirs: Vec::new() };

        for dir in dirs {
            match DirWithFiles::new(&dir) {
                Ok(dir_with_files) => {
                    files_in_dirs.dirs.push(dir_with_files);
                }
                Err(e) => {
                    eprintln!("Error reading directory '{}': {}", dir, e);
                    continue; // Skip this directory if it cannot be read
                }
            };
        }

        files_in_dirs
    }

    pub fn load_from_disk(path: std::path::PathBuf) -> Result<Self, String> {
        if path.exists() {
            let data =
                std::fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
            serde_json::from_str(&data).map_err(|e| format!("Failed to parse JSON: {}", e))
        } else {
            Ok(FilesInDirs { dirs: Vec::new() })
        }
    }

    // TODO: Result unnecessary?
    // TODO: Make disk write async, dont make ui wait for it
    pub fn save_to_disk(&self, path: std::path::PathBuf) -> Result<(), String> {
        let data = serde_json::to_string(self)
            .map_err(|e| format!("Failed to serialize FilesInDirs: {}", e))?;

        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        std::fs::write(&path, data).map_err(|e| format!("Failed to write to disk: {}", e))
    }

    // TODO: Add functions add_dir, rm_dir, rescan_dir, rescan_all
    pub fn add_dir(&mut self, dir: String) -> Result<(), String> {
        let dir_with_files = DirWithFiles::new(&dir)?;
        self.dirs.push(dir_with_files);
        Ok(())
    }

    pub fn remove_dir(&mut self, dir: &String) -> Result<(), String> {
        if let Some(pos) = self.dirs.iter().position(|d| &d.path == dir) {
            self.dirs.remove(pos);
            Ok(())
        } else {
            Err(format!("Directory '{}' not found", dir))
        }
    }
}
