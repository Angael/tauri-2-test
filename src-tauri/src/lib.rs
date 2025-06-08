// Declare the todo module
pub mod commands;
pub mod saved_folders;
pub mod todo;

use commands::{
    add_todo, get_todos, greet, load_todos_from_disk, remove_todo, toggle_todo, AppState,
};
use std::sync::Mutex;
use tauri::Manager;

use crate::saved_folders::load_folders_from_disk;

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
            let saved_folders_path = app_data_dir.join("saved_folders.json");

            // Print paths
            println!("Data file path: {}", data_file_path.display());

            // Load initial todos and determine the next ID
            let initial_todos = load_todos_from_disk(&data_file_path);
            let max_id = initial_todos.iter().map(|t| t.id).max().unwrap_or(0);

            let saved_folders = Mutex::new(load_folders_from_disk(&saved_folders_path));

            // Create and manage the application state
            let app_state = AppState {
                data_file_path,
                saved_folders_path,

                saved_folders,
                todos: Mutex::new(initial_todos),
                next_id: Mutex::new(max_id + 1),
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
            saved_folders::save_folders,
            saved_folders::get_saved_folders,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
