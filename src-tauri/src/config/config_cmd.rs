use crate::save_load::SaveLoad;
use crate::{app_state::AppState, config::AppConfig};

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn get_config(state: tauri::State<AppState>) -> Result<AppConfig, String> {
    println!("get_config");

    let config = state
        .app_config
        .lock()
        .map_err(|e| format!("Failed to lock app config: {}", e))?;

    Ok(config.clone())
}

#[tauri::command]
pub fn set_config(
    ffmpeg_path: String,
    ffprobe_path: String,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    println!("set_config");

    let mut config = state
        .app_config
        .lock()
        .map_err(|e| format!("Failed to lock app config: {}", e))?;

    config.ffmpeg_path = ffmpeg_path;
    config.ffprobe_path = ffprobe_path;
    config.save_to_disk()?;

    Ok(())
}
