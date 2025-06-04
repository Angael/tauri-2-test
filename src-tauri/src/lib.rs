// Declare the todo module
pub mod commands;
pub mod todo;

use commands::{
    add_todo, get_todos, greet, load_todos_from_disk, remove_todo, toggle_todo, AppState,
};
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

            // Pring path
            println!("Data file path: {}", data_file_path.display());

            // Load initial todos and determine the next ID
            let initial_todos = load_todos_from_disk(&data_file_path);
            let max_id = initial_todos.iter().map(|t| t.id).max().unwrap_or(0);

            // Create and manage the application state
            let app_state = AppState {
                todos: Mutex::new(initial_todos),
                data_file_path,
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
            remove_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
