use dotenv;
use glob::glob;
use image_base64::to_base64;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[path = "types/tileset.rs"]
mod tileset_json;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder]
}

#[tauri::command]
pub fn read_tileset_folder(tileset_path_name: &str) -> tileset_json::CDDATileSetConfigWithCache {
  let mut files_in_folder: Vec<std::string::String> = vec![];
  for entry in glob(&format!("{}/*", tileset_path_name)).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        files_in_folder.push(path.into_os_string().into_string().unwrap());
      }
      Err(e) => println!("{:?}", e),
    }
  }
  // read tile_config file
  let config_file_path = format!("{}/tile_config.json", tileset_path_name);
  let mut raw_tile_config_file = File::open(config_file_path).unwrap();
  let mut raw_tile_config_string = String::new();
  raw_tile_config_file.read_to_string(&mut raw_tile_config_string).unwrap();
  let raw_tile_config: tileset_json::CDDATileSetConfig = serde_json::from_str(&raw_tile_config_string).unwrap();
  // prepare textures
  let mut textures: BTreeMap<String, String> = BTreeMap::new();
  for tile_config_item in &raw_tile_config.tiles_new {
    let texture_image_file_path = format!("{}/{}", tileset_path_name, tile_config_item.file);
    textures.insert(tile_config_item.file.clone(), to_base64(&texture_image_file_path));
  }
  let raw_config = tileset_json::CDDATileSetConfigWithCache {
    raw_config: raw_tile_config,
    textures: textures,
  };
  raw_config
}
