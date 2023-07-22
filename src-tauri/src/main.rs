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

#[tauri::command]
fn load_start_time() -> u8 {
    info!("Loading ID");
    let video_id: u8 = (std::fs::read_to_string("start_time.txt")
        .unwrap_or(String::from("0")) // if file cannot be found
        .trim())
    .parse::<u8>()
    .unwrap_or(0); //If Text cannot be parsed
    video_id
}

fn main() {
    pretty_env_logger::init_timed();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_video_id, load_start_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
