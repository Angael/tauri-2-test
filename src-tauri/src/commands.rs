// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Add these imports at the top of src-tauri/src/main.rs
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::todo::Todo;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Helper function to load todos from disk
pub fn load_todos_from_disk(path: &PathBuf) -> Vec<Todo> {
    if path.exists() {
        let data = fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|err| {
            eprintln!(
                "Failed to parse todos from {}: {}, starting with empty list.",
                path.display(),
                err
            );
            Vec::new()
        })
    } else {
        Vec::new()
    }
}

// Helper function to save todos to disk
pub fn save_todos_to_disk(path: &PathBuf, todos: &[Todo]) -> Result<(), String> {
    let data = serde_json::to_string_pretty(todos)
        .map_err(|e| format!("Failed to serialize todos: {}", e))?;

    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }

    fs::write(path, data).map_err(|e| format!("Failed to write todos to disk: {}", e))
}

#[tauri::command]
pub fn get_todos(state: tauri::State<AppState>) -> Result<Vec<Todo>, String> {
    Ok(state.todos.lock().unwrap().clone())
}

#[tauri::command]
pub fn add_todo(text: String, state: tauri::State<AppState>) -> Result<Todo, String> {
    let mut todos_guard = state.todos.lock().unwrap();
    let mut next_id_guard = state.next_id.lock().unwrap();

    let new_todo = Todo {
        id: *next_id_guard,
        text,
        completed: false,
    };
    todos_guard.push(new_todo.clone());
    *next_id_guard += 1;

    save_todos_to_disk(&state.data_file_path, &todos_guard)?;
    Ok(new_todo)
}

#[tauri::command]
pub fn toggle_todo(id: u32, state: tauri::State<AppState>) -> Result<Todo, String> {
    let mut todos_guard = state.todos.lock().unwrap();
    if let Some(todo) = todos_guard.iter_mut().find(|t| t.id == id) {
        todo.completed = !todo.completed;
        let cloned_todo = todo.clone();
        save_todos_to_disk(&state.data_file_path, &todos_guard)?;
        Ok(cloned_todo)
    } else {
        Err(format!("Todo with id {} not found", id))
    }
}

#[tauri::command]
pub fn remove_todo(id: u32, state: tauri::State<AppState>) -> Result<(), String> {
    let mut todos_guard = state.todos.lock().unwrap();
    let initial_len = todos_guard.len();
    todos_guard.retain(|t| t.id != id);

    if todos_guard.len() < initial_len {
        save_todos_to_disk(&state.data_file_path, &todos_guard)?;
        Ok(())
    } else {
        Err(format!("Todo with id {} not found for removal", id))
    }
}
