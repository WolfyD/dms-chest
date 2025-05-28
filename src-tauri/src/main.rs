// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

use database::{get_database_state, initialize_database};

fn main() {
    tauri::Builder::default()
        .manage(get_database_state())
        .invoke_handler(tauri::generate_handler![
            initialize_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
