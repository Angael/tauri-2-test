// Declare the todo module
pub mod app_state;
pub mod commands;
pub mod config;
pub mod files_in_dirs;
pub mod save_load;
pub mod todo;

use crate::save_load::SaveLoad;

use crate::{app_state::AppState, config::AppConfig, files_in_dirs::model::FilesInDirs};
use commands::{add_todo, get_todos, greet, load_todos_from_disk, remove_todo, toggle_todo};
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
            let data_file_path = app_data_dir.join("todos.json");
            let files_in_dirs_path = app_data_dir.join("files_in_dirs.json");

            // Print paths
            println!("app_data_dir: {}", app_data_dir.display());

            // Load initial todos and determine the next ID
            let initial_todos = load_todos_from_disk(&data_file_path);
            let max_id = initial_todos.iter().map(|t| t.id).max().unwrap_or(0);

            // Create and manage the application state
            let app_state = AppState {
                // Paths
                data_file_path,
                files_in_dirs_path: files_in_dirs_path.clone(),

                // State
                app_config: AppConfig::load_from_disk(),
                files_in_dirs: Mutex::new(
                    FilesInDirs::load_from_disk(files_in_dirs_path)
                        .expect("Failed to load files in directories"),
                ),
                todos: Mutex::new(initial_todos), // Deprecated
                next_id: Mutex::new(max_id + 1),  // Deprecated
            };
            app.manage(app_state); // Make AppState available to commands

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_todos,
            add_todo,
            toggle_todo,
            remove_todo,
            // TODO: add "use" to shorten these, make sure commands files are named differently
            files_in_dirs::commands::get_files_in_dirs,
            files_in_dirs::commands::add_dir,
            files_in_dirs::commands::remove_dir,
            config::config_commands::get_config,
            config::config_commands::set_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
