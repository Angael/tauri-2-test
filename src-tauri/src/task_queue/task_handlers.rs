use ffprobe::ffprobe;
use std::path::{Path, PathBuf};
use tauri::{Emitter, Manager};

use crate::{
    app_state::AppState,
    task_queue::task::{AnalyzeVideoTask, GenerateThumbTask},
};

const VIDEO_EXTENSIONS: [&str; 6] = [".mp4", ".mkv", ".avi", ".mov", ".flv", ".wmv"];

pub fn handle_task_analyze_video(task: AnalyzeVideoTask, app_handle: tauri::AppHandle) {
    let is_analyzed: bool = app_handle.state::<AppState>().files_in_dirs.with(|s| {
        for dir in s.dirs.iter() {
            if dir.path == task.dir {
                if let Some(file) = dir.files.iter().find(|f| f.name == task.file) {
                    // Check if the file has already been analyzed
                    return file.video_stats.is_some();
                }
            }
        }
        false
    });

    if is_analyzed {
        println!(
            "Video '{}' in directory '{}' has already been analyzed.",
            task.file, task.dir
        );
        return;
    }

    let path: PathBuf = Path::new(&task.dir).join(&task.file);

    match ffprobe(path) {
        Ok(info) => {
            println!("Pretty ffprobe info: {:#?}", info);
        }
        Err(e) => {
            eprintln!("Failed to analyze video '{}': {}", task.file, e);
        }
    }
}

pub fn handle_task_generate_thumb(task: GenerateThumbTask, app_handle: tauri::AppHandle) {
    // println!("Handling GenerateThumb task for file: {}", task.file);

    let is_video: bool = VIDEO_EXTENSIONS
        .iter()
        .any(|ext| task.file.to_lowercase().ends_with(ext));

    if !is_video {
        println!("Skipping non-video file: {}", task.file);
        return;
    }

    let _file_in_state = app_handle.state::<AppState>().files_in_dirs.with(|s| {
        for dir in s.dirs.iter() {
            if dir.path == task.dir {
                if let Some(file) = dir.files.iter().find(|f| f.name == task.file) {
                    // Here you could add logic to analyze the video file if needed.
                    // For example, you might want to extract metadata or perform some checks.
                    println!("Found video file: {} in directory: {}", file.name, task.dir);
                    return Some(file.clone());
                } else {
                    println!("File {} not found in directory {}", task.file, task.dir);
                    return None;
                }
            }
        }

        return None;
    });

    // Here you would implement the logic to generate the thumbnail.
    // For example, you might call a function that uses FFmpeg to create the thumbnail.
    // gen_ffmpeg_vid_tiled_thumb(dir, file);

    // Sleep for 500ms
    // thread::sleep(std::time::Duration::from_millis(200));

    // Simulate a successful operation
    // println!(
    //     "Thumbnail generated for {} in directory {}",
    //     task.file, task.dir
    // );

    // Example: Emit the processed event to the frontend
    app_handle.emit("task_generate_thumb", task).unwrap();
}
