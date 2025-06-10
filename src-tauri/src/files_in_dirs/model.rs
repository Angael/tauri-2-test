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
}
