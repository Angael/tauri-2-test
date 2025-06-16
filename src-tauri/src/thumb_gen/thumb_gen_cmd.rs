use std::path::Path;

use crate::{app_state::AppState, thumb_gen::thumb_gen::do_ffmpeg_stuff};

#[tauri::command]
pub fn generate_thumbnails(state: tauri::State<AppState>, dir: String) -> Result<(), String> {
    println!("generate_thumbnails: {:?}", dir);
    let files = state
        .files_in_dirs
        .with(|files_in_dirs| files_in_dirs.dirs.iter().find(|d| d.path == dir).cloned());

    if files.is_none() {
        return Err(format!("Directory '{}' not found in files_in_dirs", dir));
    }

    let unwrapped = files.unwrap();
    let files = unwrapped.files;
    let dir_path = Path::new(&unwrapped.path);

    for file in files.iter() {
        // Join two paths:
        let file_path = dir_path.join(&file.name);
        println!("Generating thumbnail for file: {}", file.name);
        let output = do_ffmpeg_stuff(file_path.to_str().unwrap().to_string());

        println!("FFmpeg output: {:?}", output);
    }

    Ok(())
}
