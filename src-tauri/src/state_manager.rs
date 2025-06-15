// src/state_manager.rs
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock}; // Using RwLock is often better for state

#[derive(Debug)]
pub struct JsonState<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    path: PathBuf,
    state: Arc<RwLock<T>>,
}

impl<T> JsonState<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    /// Loads state from a JSON file, or creates a default state if the file
    /// doesn't exist or is invalid.
    pub fn load(path: PathBuf) -> Self {
        let state = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_else(|| {
                    eprintln!("Failed to read or parse JSON file at: {}", path.display());
                    T::default()
                })
        } else {
            T::default()
        };

        return Self {
            path,
            state: Arc::new(RwLock::new(state)),
        };
    }

    /// Saves the current state to its JSON file.
    /// This operation acquires a read lock on the state.
    pub fn save(&self) -> Result<(), String> {
        if let Some(parent_dir) = self.path.parent() {
            fs::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let state_guard = self.state.read().map_err(|e| e.to_string())?;
        let data = serde_json::to_string_pretty(&*state_guard)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;

        fs::write(&self.path, data).map_err(|e| format!("Failed to write to disk: {}", e))
    }

    /// Provides safe, read-only access to the state via a closure.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let state_guard = self.state.read().unwrap();
        f(&*state_guard)
    }

    /// Provides safe, mutable access to the state via a closure.
    /// After the closure finishes, the state is saved to disk automatically.
    pub fn with_mut<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut T) -> R,
    {
        let result = {
            let mut state_guard = self.state.write().unwrap();
            f(&mut *state_guard)
        };

        // Automatically save after a mutation
        if let Err(e) = self.save() {
            // Handle or log the save error appropriately
            eprintln!("Error saving {}: {}", self.path.display(), e);
            return Err(e);
        }

        Ok(result)
    }
}

// Allow cloning the handle to the state, not the state itself.
impl<T> Clone for JsonState<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            state: self.state.clone(),
        }
    }
}
