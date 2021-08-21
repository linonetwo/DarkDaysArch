use glob::glob;
use std::fs::File;
use std::io::Read;

#[path = "types/tileset.rs"]
mod tileset_json;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder]
}

#[tauri::command]
pub fn read_tileset_folder(path_name: &str) -> tileset_json::CDDATileSetConfigWithCache {
  let mut files_in_folder: Vec<std::string::String> = vec![];
  for entry in glob(&format!("{}/*", path_name)).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        files_in_folder.push(path.into_os_string().into_string().unwrap());
      }
      Err(e) => println!("{:?}", e),
    }
  }
  let mut raw_tile_config_file = File::open(
    "/Users/linonetwo/Desktop/repo/DarkDaysArch/public/assets/ChibiUltica/tile_config.json",
  )
  .unwrap();
  let mut raw_tile_config_string = String::new();
  raw_tile_config_file
    .read_to_string(&mut raw_tile_config_string)
    .unwrap();
  let raw_tile_config: tileset_json::CDDATileSetConfig =
    serde_json::from_str(&raw_tile_config_string).unwrap();
  let raw_config = tileset_json::CDDATileSetConfigWithCache {
    raw_config: raw_tile_config,
  };
  raw_config
}
