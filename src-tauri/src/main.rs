// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tauri::command]
fn load_video_id() -> String {
    info!("Loading ID");
    let video_id: String = String::from(
        std::fs::read_to_string("id.txt")
            .unwrap_or(String::from("dQw4w9WgXcQ"))
            .trim(),
    );
    video_id
}

fn main() {
    pretty_env_logger::init_timed();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_video_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
