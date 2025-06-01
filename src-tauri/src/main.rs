// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod extractor;

use database::{get_database_state, initialize_database, query_database};
use extractor::extract_7z;
use std::fs;

#[tauri::command]
async fn extract_executables(app: tauri::AppHandle) -> Result<(), String> {
    extract_7z(&app)
}

#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(get_database_state())
        .invoke_handler(tauri::generate_handler![
            initialize_database,
            extract_executables,
            query_database,
            read_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
