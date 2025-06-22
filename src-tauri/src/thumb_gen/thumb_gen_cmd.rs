use crate::{
    app_state::AppState,
    task_queue::task::{GenerateThumbTask, Task},
};

#[tauri::command]
pub fn generate_thumbnails(state: tauri::State<AppState>, dir: String) -> Result<(), String> {
    println!("generate_thumbnails: {:?}", dir);
    let files = state
        .files_in_dirs
        .with(|files_in_dirs| files_in_dirs.dirs.iter().find(|d| d.path == dir).cloned());

    if files.is_none() {
        return Err(format!("Directory '{}' not found in files_in_dirs", dir));
    }

    let unwrapped = files.unwrap();
    let files = unwrapped.files;

    for file in files.iter() {
        println!("generate_thumbnails: {}", file.name);

        todo!("Implement manual thumbnail generation?");
        // state
        //     .event_queue
        //     .enqueue(Task::GenerateThumb(GenerateThumbTask {
        //         dir: dir.clone(),
        //         id: file.id.clone(),
        //     }));
    }

    Ok(())
}
