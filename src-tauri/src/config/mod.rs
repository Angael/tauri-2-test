use serde::{Deserialize, Serialize};

pub mod config_cmd;
pub mod nvidia_detection;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppConfig {
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
}
