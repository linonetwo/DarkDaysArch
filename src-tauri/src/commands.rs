use glob::glob;
use image_base64::to_base64;
use regex::Regex;
use std::collections::BTreeMap;
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
  // prepare inverse index
  let id_start_matching_regex = Regex::new(r"range (\d+) to (\d+)").unwrap();
  let mut tile_data_index: BTreeMap<String, tileset_json::CDDATileSetInverseIndexedTileData> = BTreeMap::new();
  for tile_config_item in &raw_tile_config.tiles_new {
    let mut cloned_tile_config_item = tile_config_item.clone();
    cloned_tile_config_item.tiles = vec![];
    cloned_tile_config_item.ascii = vec![];
    cloned_tile_config_item.sprite_height = Some(cloned_tile_config_item.sprite_height.unwrap_or(raw_tile_config.tile_info[0].height));
    cloned_tile_config_item.sprite_width = Some(cloned_tile_config_item.sprite_width.unwrap_or(raw_tile_config.tile_info[0].width));
    cloned_tile_config_item.sprite_offset_x = Some(cloned_tile_config_item.sprite_offset_x.unwrap_or(0));
    cloned_tile_config_item.sprite_offset_y = Some(cloned_tile_config_item.sprite_offset_y.unwrap_or(0));
    cloned_tile_config_item.comment = Some(cloned_tile_config_item.comment.unwrap_or(String::from("range 1 to 63")));
    let id_start_matching_result: i64 = id_start_matching_regex
      .captures(&cloned_tile_config_item.comment.clone().unwrap())
      .unwrap()
      .get(1)
      .unwrap()
      .as_str()
      .parse::<i64>()
      .unwrap();
    for tile_data_item in &tile_config_item.tiles {
      match &tile_data_item.id {
        tileset_json::CDDATileSetID::Id(tile_id_string) => {
          tile_data_index.insert(
            tile_id_string.clone(),
            tileset_json::CDDATileSetInverseIndexedTileData {
              tile: tile_data_item.clone(),
              tileset: cloned_tile_config_item.clone(),
              start_id: id_start_matching_result,
            },
          );
        }
        tileset_json::CDDATileSetID::IdList(tile_id_list) => {
          for tile_id_string_in_list in tile_id_list {
            tile_data_index.insert(
              tile_id_string_in_list.clone(),
              tileset_json::CDDATileSetInverseIndexedTileData {
                tile: tile_data_item.clone(),
                tileset: cloned_tile_config_item.clone(),
                start_id: id_start_matching_result,
              },
            );
          }
        }
      }
    }
  }
  let config_with_cache = tileset_json::CDDATileSetConfigWithCache {
    textures: textures,
    tile_data_index: tile_data_index,
  };
  config_with_cache
}
