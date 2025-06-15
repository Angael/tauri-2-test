use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Mutex;

// Define a trait for savable and loadable data
pub trait SaveLoad: Serialize + for<'de> Deserialize<'de> + Sized + Default {
    fn file_name() -> &'static str;

    fn save_to_disk(&self) -> Result<(), String> {
        let path = Path::new(Self::file_name());
        let serialized = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize data: {}", e))?;
        fs::write(path, serialized).map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }

    fn load_from_disk() -> Mutex<Self> {
        let path = Path::new(Self::file_name());
        let content = fs::read_to_string(path).unwrap_or_else(|_| String::new());
        return Mutex::new(serde_json::from_str(&content).unwrap_or_else(|_| Self::default()));
    }

    // TODO maybe solution to not passing path is to add fn init, that takes in absolute path and sets it in state, just make sure its never serialized? not sure
}
