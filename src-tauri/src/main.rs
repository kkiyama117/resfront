// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![cfg_attr(not(feature = "alloc"), )]

mod entities;

use tauri::Manager;
use crate::entities::args::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: GreetArgs<'_>) -> Result<GreetResult, String> {
    Ok(GreetResult {
        res: format!("Hello, {}! You've been greeted from Rust!", name.name)
    })
}

fn main() {
    tauri::Builder::default()
        .setup(
            |app| {
                #[cfg(debug_assertions)]
                {
                    let window = app.get_window("main").unwrap();
                    window.open_devtools();
                }
                Ok(())
            }
        )
        .invoke_handler(tauri::generate_handler![
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
