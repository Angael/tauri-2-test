use ffprobe::ffprobe;
use std::path::{Path, PathBuf};
use tauri::{Emitter, Manager};

use crate::{
    app_state::AppState,
    files_in_dirs::file::VideoStats,
    task_queue::task::{AnalyzeVideoTask, GenerateThumbTask},
};

const VIDEO_EXTENSIONS: [&str; 6] = [".mp4", ".mkv", ".avi", ".mov", ".flv", ".wmv"];

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
    if !is_video_file(&task.file) {
        println!("Skipping non-video file for analysis: {}", task.file);
        return;
    }

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

    let info = match ffprobe(path) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to analyze video '{}': {}", task.file, e);
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
            eprintln!("No video stream found in '{}'", task.file);
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
        let target_file = s
            .dirs
            .iter_mut()
            .find(|dir| dir.path == task.dir)
            .and_then(|dir| dir.files.iter_mut().find(|f| f.name == task.file));

        if let Some(file) = target_file {
            file.video_stats = Some(video_stats.clone());
            println!(
                "Video stats for '{}' in directory '{}' updated: {:?}",
                task.file, task.dir, video_stats
            );
        } else {
            eprintln!("File '{}' not found in directory '{}'", task.file, task.dir);
        }
    });
}

pub fn handle_task_generate_thumb(task: GenerateThumbTask, app_handle: &tauri::AppHandle) {
    // println!("Handling GenerateThumb task for file: {}", task.file);

    if !is_video_file(&task.file) {
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

    // get cache dir to put thumbs in:
    app_handle.path().app_cache_dir();

    // create names for thumb with nanoid
    todo!("Implement thumbnail generation logic here");

    // write thumbnail to file info
    // For example, you might want to store the thumbnail path in the file's metadata.

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
