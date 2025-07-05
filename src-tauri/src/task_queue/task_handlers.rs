use std::path::Path;
use tauri::{Emitter, Manager};

use crate::{
    app_state::AppState,
    task_queue::task::GenerateThumbTask,
    thumb_gen::thumb_gen::{gen_ffmpeg_vid_tiled_thumb, gen_image_thumb},
};

const VIDEO_EXTENSIONS: [&str; 7] = [".webm", ".mp4", ".mkv", ".avi", ".mov", ".flv", ".wmv"];
const IMAGE_EXTENSIONS: [&str; 8] = [
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff", ".webp", ".avif",
];

#[derive(PartialEq)]
enum FileType {
    Video,
    Image,
    Other,
}

fn get_file_type(filename: &str) -> FileType {
    let lower_filename = filename.to_lowercase();

    if VIDEO_EXTENSIONS
        .iter()
        .any(|ext| lower_filename.ends_with(ext))
    {
        return FileType::Video;
    }

    if IMAGE_EXTENSIONS
        .iter()
        .any(|ext| lower_filename.ends_with(ext))
    {
        return FileType::Image;
    }

    FileType::Other
}

pub fn handle_task_generate_thumb(task: GenerateThumbTask, app_handle: &tauri::AppHandle) {
    let file: crate::files_in_dirs::file::File = match app_handle
        .state::<AppState>()
        .files_in_dirs
        .with(|s| s.find_file(&task.dir, &task.id).cloned())
    {
        Some(file) => file,
        None => {
            eprintln!(
                "File with ID '{}' not found in directory '{}'",
                task.id, task.dir
            );
            return;
        }
    };

    let file_type = get_file_type(&file.name);
    if file_type == FileType::Other {
        println!("Skipping \"other\" file: {}", file.name);
        app_handle.emit("dir_scan_progress", task).unwrap();
        return;
    }

    // get cache dir to put thumbs in:
    let thumbnail_dir = &app_handle.state::<AppState>().thumbnail_store.dir;

    let input_path_str = Path::new(&task.dir)
        .join(&file.name)
        .to_string_lossy()
        .to_string();

    let thumb = match file_type {
        FileType::Video => gen_ffmpeg_vid_tiled_thumb(&file, input_path_str, &thumbnail_dir),
        FileType::Image => gen_image_thumb(input_path_str, &thumbnail_dir),
        _ => Err("Unsupported file type".to_string()),
    };

    if let Ok(thumb_data) = thumb {
        println!("thumb data {} {:?}", file.name, thumb_data);
        let _ = app_handle.state::<AppState>().files_in_dirs.with_mut(|s| {
            if let Some(file) = s.find_file_mut(&task.dir, &task.id) {
                file.thumbs.push(thumb_data);
            } else {
                eprintln!("File '{}' not found in directory '{}'", file.name, task.dir);
            }
        });
    }

    app_handle.emit("dir_scan_progress", task).unwrap();
}
