// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

// Load Json Config

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    video_id: String,
    start_time: u32,
}

impl Config {
    fn default() -> Self {
        Config {
            video_id: String::from("Ct6BUPvE2sM"),
            start_time: 14,
        }
    }
}

#[tauri::command]
fn load_config() -> Config {
    info!("Loading Config");

    let config_string = std::fs::read_to_string("config.json");

    let config: Config = match config_string {
        // If file could be read, try to parse the file and return the struct
        Ok(contents) => {
            let config = serde_json::from_str(&contents);
            match config {
                // If parsing the config file worked, return the config
                Ok(config) => config,
                // If parsing the file fails, return the default config
                Err(_) => Config::default(),
            }
        }
        // If reading the file fails, return the default config
        Err(_) => Config::default(),
    };

    config
}

fn main() {
    pretty_env_logger::init_timed();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
