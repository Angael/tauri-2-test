// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// When testing
// mod test;

fn main() {
    // When testing
    // test::run();

    // When developing
    tauri_2_test_lib::run()
}
