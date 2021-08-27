#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#[allow(unused_imports)]

mod commands;
mod parsers;
mod utils;

fn main() {
  tauri::Builder::default()
    .invoke_handler(commands::invoke_handler())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
