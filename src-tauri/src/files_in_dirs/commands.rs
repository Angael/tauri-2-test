use crate::{app_state::AppState, files_in_dirs::model::FilesInDirs};

#[tauri::command]
pub fn get_files_in_dirs(state: tauri::State<AppState>) -> Result<FilesInDirs, String> {
    println!("get_files_in_dirs");

    let state = state
        .files_in_dirs
        .lock()
        .map_err(|e| format!("Failed to lock files in dirs: {}", e))?;

    Ok(state.clone())
}

#[tauri::command]
pub fn add_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("add_dir: {:?}", dir);

    let mut files_in_dirs = state
        .files_in_dirs
        .lock()
        .map_err(|e| format!("Failed to lock files in dirs: {}", e))?;

    files_in_dirs.add_dir(dir)?;
    files_in_dirs.save_to_disk(state.files_in_dirs_path.clone());

    Ok(())
}

#[tauri::command]
pub fn remove_dir(dir: String, state: tauri::State<AppState>) -> Result<(), String> {
    println!("remove_dir: {:?}", dir);

    let mut files_in_dirs = state
        .files_in_dirs
        .lock()
        .map_err(|e| format!("Failed to lock files in dirs: {}", e))?;

    files_in_dirs.remove_dir(&dir)?;
    files_in_dirs.save_to_disk(state.files_in_dirs_path.clone());

    Ok(())
}
