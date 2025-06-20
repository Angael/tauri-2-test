// src/state_manager.rs
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct JsonState<T>
where
    T: Serialize + DeserializeOwned + Default + Send + Sync + 'static,
{
    path: PathBuf,
    state: Arc<RwLock<T>>,
    save_trigger: Sender<SaveRequest>,
}

#[derive(Debug)]
struct SaveRequest {
    timestamp: Instant,
    force: bool, // true for safety saves, false for regular debounced saves
}

impl<T> JsonState<T>
where
    T: Serialize + DeserializeOwned + Default + Send + Sync + 'static,
{
    /// Loads state from a JSON file, or creates a default state if the file
    /// doesn't exist or is invalid.
    pub fn load(path: PathBuf) -> Self {
        let path_with_ext = path.with_extension("msgpack");
        let state = if path_with_ext.exists() {
            println!("Loading state from: {}", path_with_ext.display());

            // JSON way requires fs::read_to_string, and then json_serde::from_string or something like that
            fs::read(&path_with_ext)
                .ok()
                .and_then(|content| rmp_serde::from_slice(&content).ok())
                .unwrap_or_else(|| {
                    eprintln!("Failed to read or parse JSON file at: {}", path.display());
                    T::default()
                })
        } else {
            T::default()
        };

        let state = Arc::new(RwLock::new(state));
        let (tx, rx) = mpsc::channel();

        // Start background save thread
        let state_clone = Arc::clone(&state);
        let path_clone = path.clone();
        thread::spawn(move || {
            Self::background_save_loop(rx, state_clone, path_clone);
        });

        Self {
            path,
            state,
            save_trigger: tx,
        }
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

        // for testing purposes, also save as msgpack
        let msgpack_data = rmp_serde::to_vec(&*state_guard)
            .map_err(|e| format!("Failed to serialize state to msgpack: {}", e))?;
        let msgpack_path = self.path.with_extension("msgpack");

        println!("Saving state");
        fs::write(self.path.with_extension("json"), data)
            .map_err(|e| format!("Failed to write to disk: {}", e))?;

        fs::write(&msgpack_path, msgpack_data)
            .map_err(|e| format!("Failed to write msgpack to disk: {}", e))
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
    /// After the closure finishes, a debounced save is triggered.
    pub fn with_mut<F, R>(&self, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut T) -> R,
    {
        let result = {
            let mut state_guard = self.state.write().unwrap();
            f(&mut *state_guard)
        };

        // Trigger a debounced save (non-blocking)
        let _ = self.save_trigger.send(SaveRequest {
            timestamp: Instant::now(),
            force: false,
        });

        Ok(result)
    }

    /// Forces an immediate save (useful for shutdown or critical operations)
    pub fn force_save(&self) -> Result<(), String> {
        let _ = self.save_trigger.send(SaveRequest {
            timestamp: Instant::now(),
            force: true,
        });
        Ok(())
    }

    /// Background thread that handles debounced and safety saves
    fn background_save_loop(receiver: Receiver<SaveRequest>, state: Arc<RwLock<T>>, path: PathBuf) {
        let debounce_duration = Duration::from_millis(1500); // 1.5 second debounce
        let safety_save_interval = Duration::from_secs(15); // Safety save every 15 seconds

        let mut last_save_time = Instant::now();
        let mut last_request: Option<SaveRequest> = None;

        loop {
            // Check for save requests with timeout
            let request = match receiver.recv_timeout(Duration::from_millis(100)) {
                Ok(req) => Some(req),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => break, // Channel closed, exit thread
            };

            if let Some(req) = request {
                last_request = Some(req);
            }

            let now = Instant::now();
            let should_save = if let Some(ref req) = last_request {
                // Force save immediately
                req.force ||
                // Debounced save: enough time has passed since the request
                (now.duration_since(req.timestamp) >= debounce_duration) ||
                // Safety save: too much time since last save
                (now.duration_since(last_save_time) >= safety_save_interval)
            } else {
                // Safety save only
                now.duration_since(last_save_time) >= safety_save_interval
            };

            if should_save && last_request.is_some() {
                // Drain any additional requests (they're all for the current state)
                while receiver.try_recv().is_ok() {}

                // Perform the actual save
                if let Err(e) = Self::save_state_to_disk(&state, &path) {
                    eprintln!("Background save error: {}", e);
                } else {
                    println!("Background save completed");
                    last_save_time = now;
                    last_request = None; // Clear the request after successful save
                }
            }
        }

        println!("Background save thread shutting down");
    }

    /// Internal method to save state to disk
    fn save_state_to_disk(state: &Arc<RwLock<T>>, path: &PathBuf) -> Result<(), String> {
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        let state_guard = state.read().map_err(|e| e.to_string())?;
        let data = serde_json::to_string_pretty(&*state_guard)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;

        // for testing purposes, also save as msgpack
        let msgpack_data = rmp_serde::to_vec(&*state_guard)
            .map_err(|e| format!("Failed to serialize state to msgpack: {}", e))?;
        let msgpack_path = path.with_extension("msgpack");

        fs::write(path.with_extension("json"), data)
            .map_err(|e| format!("Failed to write to disk: {}", e))?;

        fs::write(&msgpack_path, msgpack_data)
            .map_err(|e| format!("Failed to write msgpack to disk: {}", e))
    }
}

// Allow cloning the handle to the state, not the state itself.
impl<T> Clone for JsonState<T>
where
    T: Serialize + DeserializeOwned + Default + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            state: self.state.clone(),
            save_trigger: self.save_trigger.clone(),
        }
    }
}
