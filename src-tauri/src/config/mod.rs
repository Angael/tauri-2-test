use serde::{Deserialize, Serialize};

use crate::save_load::SaveLoad;

pub mod config_cmd;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppConfig {
    #[serde(skip_serializing)]
    pub _json_path: String,

    pub ffmpeg_path: String,
    pub ffprobe_path: String,
}

impl SaveLoad for AppConfig {
    fn file_name() -> &'static str {
        "app_config.json"
    }
}
