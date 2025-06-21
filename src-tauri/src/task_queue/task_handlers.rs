use ffprobe::ffprobe;
use std::{
    path::{Path, PathBuf},
    thread,
};
use tauri::{Emitter, Manager};

use crate::{
    app_state::AppState,
    files_in_dirs::file::VideoStats,
    task_queue::task::{AnalyzeVideoTask, GenerateThumbTask},
    thumb_gen::thumb_gen::gen_ffmpeg_vid_tiled_thumb,
};

const VIDEO_EXTENSIONS: [&str; 7] = [".webm", ".mp4", ".mkv", ".avi", ".mov", ".flv", ".wmv"];

fn is_video_file(filename: &str) -> bool {
    VIDEO_EXTENSIONS
        .iter()
        .any(|ext| filename.to_lowercase().ends_with(ext))
}

fn approx_video_bitrate(file_size_bytes: u64, duration_secs: f64, audio_fraction: f64) -> u32 {
    let bits = (file_size_bytes as f64) * 8.0 * (1.0 - audio_fraction);
    (bits / duration_secs).round() as u32
}

pub fn handle_task_analyze_video(task: AnalyzeVideoTask, app_handle: &tauri::AppHandle) {
    let file = match app_handle
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

    if !is_video_file(&file.name) {
        println!("Skipping non-video file for analysis: {}", file.name);
        return;
    }

    if file.video_stats.is_some() {
        return;
    }

    let path: PathBuf = Path::new(&task.dir).join(&file.name);

    let info = match ffprobe(path) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to analyze video '{}': {}", file.name, e);
            return;
        }
    };

    let v_stream = match info
        .streams
        .iter()
        .find(|s| s.codec_type == Some("video".to_string()))
    {
        Some(stream) => stream,
        None => {
            eprintln!("No video stream found in '{}'", file.name);
            return;
        }
    };

    let video_stats = VideoStats {
        dur: info
            .format
            .duration
            .as_ref()
            .and_then(|d| d.parse::<f64>().ok())
            .unwrap_or(0.0),
        res: (
            v_stream.width.unwrap_or(0) as u16,
            v_stream.height.unwrap_or(0) as u16,
        ),
        br: v_stream
            .bit_rate
            .as_ref()
            .and_then(|bit_rate| bit_rate.parse::<u32>().ok())
            .unwrap_or_else(|| {
                if let Some(duration_str) = &info.format.duration {
                    if let (Ok(size), Ok(duration)) =
                        (info.format.size.parse::<u64>(), duration_str.parse::<f64>())
                    {
                        return approx_video_bitrate(size, duration, 0.08_f64);
                    }
                }
                0
            }),
    };

    // Save the analysis result to the app state
    let _ = app_handle.state::<AppState>().files_in_dirs.with_mut(|s| {
        if let Some(file) = s.find_file_mut(&task.dir, &task.id) {
            file.video_stats = Some(video_stats.clone());
            println!("Analysis complete for: '{}'", file.name);
        } else {
            eprintln!("File '{}' not found in directory '{}'", file.name, task.dir);
        }
    });
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

    if !is_video_file(&file.name) {
        println!("Skipping non-video file: {}", file.name);
        return;
    }

    // get cache dir to put thumbs in:
    let thumbnail_dir = app_handle
        .path()
        .app_cache_dir()
        .expect("Error getting cache dir")
        .join(file.id);

    // Ensure the thumbnail directory exists
    std::fs::create_dir_all(&thumbnail_dir).expect("Failed to create thumbnail directory");

    let input_path_str = Path::new(&task.dir)
        .join(&file.name)
        .to_string_lossy()
        .to_string();

    gen_ffmpeg_vid_tiled_thumb(input_path_str, &thumbnail_dir);

    thread::sleep(std::time::Duration::from_millis(200));

    app_handle.emit("task_generate_thumb", task).unwrap();
}
