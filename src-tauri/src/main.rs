#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#[allow(unused_imports)]
use std::sync::Mutex;

mod commands;
mod parsers;
mod types;
mod utils;

fn main() {
  tauri::Builder::default()
    .manage(types::state::AppState(Mutex::new(types::state::AppStateRaw {
      knowledge_graph: data::types::CDDAKnowledgeGraph::new(),
    })))
    .invoke_handler(commands::invoke_handler())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
