use std::thread;
use tauri::Emitter;

use crate::task_queue::task::GenerateThumbTask;

pub fn handle_task_generate_thumb(task: GenerateThumbTask, app_handle: tauri::AppHandle) {
    println!("Handling GenerateThumb task for file: {}", task.file);
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
