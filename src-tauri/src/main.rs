//! PsychoQuine Tauri Application
//!
//! Desktop application for the universal quine generator.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::{generate_quine, get_version};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_quine, get_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
