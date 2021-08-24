#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod commands;

fn main() {
  tauri::Builder::default()
    .invoke_handler(commands::invoke_handler())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
