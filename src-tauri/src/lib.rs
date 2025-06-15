// Declare the todo module
pub mod app_state;
pub mod config;
pub mod files_in_dirs;
pub mod save_load;

use crate::save_load::SaveLoad;

use crate::{app_state::AppState, config::AppConfig, files_in_dirs::model::FilesInDirs};
use std::sync::Mutex;
use tauri::Manager;

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
            let files_in_dirs_path = app_data_dir.join("files_in_dirs.json");

            // Print paths
            println!("app_data_dir: {}", app_data_dir.display());

            // Create and manage the application state
            let app_state = AppState {
                // Paths
                files_in_dirs_path: files_in_dirs_path.clone(),

                // State
                app_config: AppConfig::load_from_disk(),
                files_in_dirs: Mutex::new(
                    FilesInDirs::load_from_disk(files_in_dirs_path)
                        .expect("Failed to load files in directories"),
                ),
            };
            app.manage(app_state); // Make AppState available to commands

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // TODO: add "use" to shorten these, make sure commands files are named differently
            files_in_dirs::files_in_dirs_cmd::get_files_in_dirs,
            files_in_dirs::files_in_dirs_cmd::add_dir,
            files_in_dirs::files_in_dirs_cmd::remove_dir,
            config::config_cmd::get_config,
            config::config_cmd::set_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
