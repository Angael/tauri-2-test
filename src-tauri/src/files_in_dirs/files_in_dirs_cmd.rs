use crate::{
    app_state::AppState,
    files_in_dirs::model::{DirWithFiles, FilesInDirs},
};
use std::fs;

#[tauri::command]
pub fn get_files_in_dirs(state: tauri::State<AppState>) -> FilesInDirs {
    println!("get_files_in_dirs");

    state
        .files_in_dirs
        .with(|files_in_dirs| files_in_dirs.clone())
}

#[tauri::command]
pub fn get_dir(dir: String, state: tauri::State<AppState>) -> Option<DirWithFiles> {
    println!("get_dir");

    state
        .files_in_dirs
        .with(|files_in_dirs| files_in_dirs.dirs.iter().find(|d| d.path.eq(&dir)).cloned())
}

#[tauri::command]
pub fn add_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("add_dir: {:?}", dir);

    state
        .files_in_dirs
        .with_mut(|files_in_dirs| files_in_dirs.add_dir(dir, &state))?
}

#[tauri::command]
pub fn remove_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("remove_dir: {:?}", dir);

    let mut file_ids_to_delete: Vec<String> = Vec::new();
    let _ = state.files_in_dirs.with_mut(|files_in_dirs| {
        // Collect file IDs to delete
        if let Some(dir) = files_in_dirs.dirs.iter().find(|d| d.path.eq(&dir)) {
            for file in &dir.files {
                file_ids_to_delete.push(file.id.clone());
            }
        }

        files_in_dirs.remove_dir(&dir)
    })?;

    for file_id in &file_ids_to_delete {
        let result = fs::remove_dir_all(&state.thumbnail_store.get_file_dir(file_id));
        match result {
            Err(err) => {
                eprintln!("Err removing thumb dir {}, {}", file_id, err);
            }
            _ => (),
        };
    }

    Ok(())
}

#[tauri::command]
pub fn rescan_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("rescan_dir: {:?}", dir);

    remove_dir(dir.clone(), state.clone())?;

    state
        .files_in_dirs
        .with_mut(|files_in_dirs| files_in_dirs.rescan_dir(&dir, &state))?
}
