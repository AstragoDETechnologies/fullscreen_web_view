// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Filesystem access
// use std::fs::File;
// use std::io::prelude::*;

// Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tauri::command]
fn greet(name: &str) -> String {
    // let mut file = File::create("file.txt").unwrap();
    // file.write_all(name.as_bytes()).unwrap();
    format!("hello, {}", name)
}

// #[tauri::command]
// fn generate_image() {
//     const IMAGE_DIMENSIONS: u32 = 1000;

//     let color_factor: f32 = 256.0 / IMAGE_DIMENSIONS as f32;
//     debug!("color_factor: {color_factor}");

//     let mut imgbuf = image::ImageBuffer::new(IMAGE_DIMENSIONS, IMAGE_DIMENSIONS);

//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r: u8 = (color_factor * x as f32) as u8;
//         let g: u8 = (((256.0 - (color_factor * x as f32)) + (256.0 - (color_factor * y as f32)))
//             / 2.0) as u8;

//         let b: u8 = (color_factor * y as f32) as u8;
//         trace!("X: {x}, Y: {y} --> {r}, {g}, {b}");
//         *pixel = image::Rgb([r, g, b]);
//     }

//     imgbuf.save("image.png").unwrap();
// }

#[tauri::command]
fn load_video_id() -> String {
    info!("Loading URL");
    let video_url: String = String::from(
        std::fs::read_to_string("id.txt")
            .unwrap_or(String::from("dQw4w9WgXcQ"))
            .trim(),
    );
    video_url
}

fn main() {
    pretty_env_logger::init_timed();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_video_id,
            // generate_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
