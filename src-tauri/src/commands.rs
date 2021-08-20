use glob::glob;
use std::path::PathBuf;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder]
}

#[tauri::command]
pub fn read_tileset_folder(path_name: &str) {
  let files_in_folder: Vec<PathBuf> = glob(path_name).unwrap().collect();
}
