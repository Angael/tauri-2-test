use crate::app_state::AppState;

#[tauri::command]
pub fn generate_thumbnails(state: tauri::State<AppState>, dir: String) -> Result<(), String> {
    println!("generate_thumbnails: {:?}", dir);
    let files = state
        .files_in_dirs
        .with(|files_in_dirs| files_in_dirs.dirs.iter().find(|d| d.path == dir).cloned());

    if let None = files {
        return Err(format!("Directory '{}' not found in files_in_dirs", dir));
    }

    for file in files.unwrap().files.iter() {
        println!("Generating thumbnail for file: {}", file.name);
    }

    todo!("Implement thumbnail generation logic");

    Ok(())
}
