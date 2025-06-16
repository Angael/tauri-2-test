use std::path::Path;

use crate::{
    app_state::AppState, task_queue::task_queue::Event,
    thumb_gen::thumb_gen::gen_ffmpeg_vid_tiled_thumb,
};

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
        println!("Processing file: {}", file.name);

        state.event_queue.enqueue(Event::Log {
            message: format!("Generating thumbnail for file: {}", file.name),
        });
        // let file_path = dir_path.join(&file.name);
        // println!("Generating thumbnail for file: {}", file_path.display());
        // let output = gen_ffmpeg_vid_tiled_thumb(file_path.to_str().unwrap().to_string());

        // println!("FFmpeg output: {:?}", output);
    }

    Ok(())
}
