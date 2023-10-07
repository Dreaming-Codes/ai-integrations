// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ocr;
mod status;

use ocr::tauri::select_area;
use status::tauri::display_status;
use status::tauri::close_status;

fn main() {
    tauri::Builder::default()
        .manage(status::tauri::StatusWindow::default())
        .invoke_handler(tauri::generate_handler![select_area, display_status, close_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
