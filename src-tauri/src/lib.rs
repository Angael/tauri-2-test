// Declare the todo module
// TODO: is pub necessary here?
pub mod app_state;
pub mod config;
pub mod files_in_dirs;
pub mod state_manager;
pub mod thumb_gen;

use crate::app_state::AppState;
use crate::state_manager::JsonState;
use ffmpeg_sidecar::command::ffmpeg_is_installed;
use tauri::Manager;

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
                println!("For demo purposes, we'll re-download and unpack it anyway.");
                println!("TIP: Use `auto_download()` to skip manual customization.");
            }

            app.manage(AppState {
                app_config: JsonState::load(app_data_dir.join("app_config.json")),
                files_in_dirs: JsonState::load(app_data_dir.join("files_in_dirs.json")),
            }); // Make AppState available to commands

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            files_in_dirs_cmd::get_files_in_dirs,
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
