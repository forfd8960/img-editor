// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod state;
mod types;
mod utils;

use state::image_state::ImageState;
use commands::export_commands;
use commands::image_commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(ImageState::new())
        .invoke_handler(tauri::generate_handler![
            image_commands::open_image,
            image_commands::apply_operation,
            image_commands::undo,
            image_commands::redo,
            export_commands::export_image_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
