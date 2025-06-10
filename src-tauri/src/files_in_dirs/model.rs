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

// TODO: move loading files from FilesInDirs to DirWithFiles::new

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesInDirs {
    pub dirs: Vec<DirWithFiles>,
}

impl FilesInDirs {
    pub fn new(dirs: Vec<String>) -> Self {
        let mut files_in_dirs = FilesInDirs { dirs: Vec::new() };

        for dir in dirs {
            let dir_clone = dir.clone();
            let entries = std::fs::read_dir(dir).unwrap_or_else(|_| {
                eprintln!("Failed to read directory: {}", dir_clone);
                std::fs::read_dir(".").unwrap() // Fallback to current directory
            });

            let mut dir_with_files = DirWithFiles {
                path: dir_clone,
                files: Vec::new(),
            };

            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_file() {
                    let metadata = path.metadata().unwrap();
                    dir_with_files.files.push(File {
                        name: path.file_name().unwrap().to_string_lossy().into_owned(),
                        size: metadata.len(),
                    });
                }
            }

            files_in_dirs.dirs.push(dir_with_files);
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
}
