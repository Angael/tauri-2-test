// Declare the todo module
// TODO: is pub necessary here?
pub mod app_state;
pub mod config;
pub mod files_in_dirs;
pub mod state_manager;
pub mod task_queue;
pub mod thumb_gen;

use crate::app_state::AppState;
use crate::state_manager::JsonState;
use crate::task_queue::task_queue::{start_event_consumer, ThreadSafeEventQueue};
use ffmpeg_sidecar::command::ffmpeg_is_installed;
use nanoid::nanoid;
use tauri::{Manager, WindowEvent};

// Import command functions to shorten generate_handler references
use crate::config::config_cmd;
use crate::files_in_dirs::files_in_dirs_cmd;
use crate::thumb_gen::thumb_gen_cmd;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone(); // Use tauri::AppHandle

            // Get the application data directory
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get application data directory. Please ensure it's configured.");

            // Print paths
            println!("app_data_dir: {}", app_data_dir.display());

            if ffmpeg_is_installed() {
                println!("FFmpeg is already installed! 🎉");
                println!("TIP: Use `auto_download()` to skip manual customization.");
            } else {
                todo!("FFmpeg is not installed. Please install it manually or use `auto_download()` to download it automatically.");
            }

            let event_queue = ThreadSafeEventQueue::new();

            // Start multiple consumers for load balancing
            for _ in 0..3 {
                let queue_for_consumer = event_queue.clone();
                let app_handle_clone = app_handle.clone();
                std::thread::spawn(move || {
                    start_event_consumer(queue_for_consumer, app_handle_clone);
                });
            }            
            
            let app_config = JsonState::load(app_data_dir.join("app_config"));
            let files_in_dirs = JsonState::load(app_data_dir.join("files_in_dirs"));

            let window = app_handle.get_webview_window("main").unwrap();
            let window_clone = window.clone();
            let app_config_clone = app_config.clone();
            let files_in_dirs_clone = files_in_dirs.clone();
            
            window.on_window_event(move |event| {
                match event {
                    WindowEvent::CloseRequested { .. } => {
                        // Perform blocking saves to ensure data persistence before shutdown
                        if let Err(e) = app_config_clone.force_save_blocking() {
                            eprintln!("Failed to save app config on shutdown: {}", e);
                        }
                        if let Err(e) = files_in_dirs_clone.force_save_blocking() {
                            eprintln!("Failed to save files_in_dirs on shutdown: {}", e);
                        }
                        
                        // Close the window after saving
                        let _ = window_clone.close();
                    }
                    _ => {}
                }
            });
            
            app.manage(AppState {
                event_queue,
                app_config,
                files_in_dirs,
            }); // Make AppState available to commands

            #[cfg(debug_assertions)] // only in debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            files_in_dirs_cmd::get_files_in_dirs,
            files_in_dirs_cmd::get_dir,
            files_in_dirs_cmd::add_dir,
            files_in_dirs_cmd::remove_dir,
            files_in_dirs_cmd::rescan_dir,
            config_cmd::get_config,
            config_cmd::set_config,
            thumb_gen_cmd::generate_thumbnails
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
