use crate::{app_state::AppState, config::AppConfig};

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn get_config(state: tauri::State<AppState>) -> Result<AppConfig, String> {
    println!("get_config");
    Ok(state.app_config.with(|config| config.clone()))
}

#[tauri::command]
pub fn set_config(
    ffmpeg_path: String,
    ffprobe_path: String,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    println!("set_config");

    state.app_config.with_mut(|config| {
        config.ffmpeg_path = ffmpeg_path;
        config.ffprobe_path = ffprobe_path;
    })
}
