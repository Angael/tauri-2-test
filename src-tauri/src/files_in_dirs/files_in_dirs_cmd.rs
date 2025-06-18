use crate::{app_state::AppState, files_in_dirs::model::FilesInDirs};

#[tauri::command]
pub fn get_files_in_dirs(state: tauri::State<AppState>) -> FilesInDirs {
    println!("get_files_in_dirs");

    state
        .files_in_dirs
        .with(|files_in_dirs| files_in_dirs.clone())
}

#[tauri::command]
pub fn add_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("add_dir: {:?}", dir);

    state
        .files_in_dirs
        .with_mut(|files_in_dirs| files_in_dirs.add_dir(dir, state))?
}

#[tauri::command]
pub fn remove_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("remove_dir: {:?}", dir);

    state
        .files_in_dirs
        .with_mut(|files_in_dirs| files_in_dirs.remove_dir(&dir))?
}

#[tauri::command]
pub fn rescan_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("rescan_dir: {:?}", dir);

    state
        .files_in_dirs
        .with_mut(|files_in_dirs| files_in_dirs.rescan_dir(&dir, state))?
}
