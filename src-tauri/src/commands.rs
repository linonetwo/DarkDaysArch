use glob::glob;
use image_base64::to_base64;
use project_root::get_project_root;
use regex::Regex;
use std::path::Path;
use std::{collections::BTreeMap, fs::File, io::Read};

#[path = "types/mapgen.rs"]
mod mapgen_json;
#[path = "types/palette.rs"]
mod palette_json;
#[path = "types/tileset.rs"]
mod tileset_json;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder, read_mapgen_file]
}

#[tauri::command]
pub fn read_tileset_folder(tileset_path_name: &str) -> tileset_json::CDDATileSetConfigWithCache {
  let tileset_absolute_file_path = Path::join(&Path::join(&get_project_root().unwrap(), "../public"), tileset_path_name).into_os_string().into_string().unwrap();
  let mut files_in_folder: Vec<std::string::String> = vec![];
  for entry in glob(&format!("{}/*", tileset_absolute_file_path)).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        files_in_folder.push(path.into_os_string().into_string().unwrap());
      }
      Err(e) => println!("{:?}", e),
    }
  }
  // read tile_config file
  let config_file_path = format!("{}/tile_config.json", tileset_absolute_file_path);
  let mut raw_tile_config_file = File::open(config_file_path).unwrap();
  let mut raw_tile_config_string = String::new();
  raw_tile_config_file.read_to_string(&mut raw_tile_config_string).unwrap();
  let raw_tile_config: tileset_json::CDDATileSetConfig = serde_json::from_str(&raw_tile_config_string).unwrap();
  // prepare textures
  let mut textures: BTreeMap<String, String> = BTreeMap::new();
  for tile_config_item in &raw_tile_config.tiles_new {
    let texture_image_file_path = format!("{}/{}", tileset_absolute_file_path, tile_config_item.file);
    textures.insert(tile_config_item.file.clone(), to_base64(&texture_image_file_path));
  }
  // prepare inverse index
  let default_tileset_info = tileset_json::CDDATileSetTileInfo {
    width: 32,
    height: 32,
    pixelscale: 1,
  };
  let id_start_matching_regex = Regex::new(r"range (\d+) to (\d+)").unwrap();
  let mut tile_data_index: BTreeMap<String, tileset_json::CDDATileSetInverseIndexedTileData> = BTreeMap::new();
  for tile_config_item in &raw_tile_config.tiles_new {
    let mut cloned_tile_config_item = tile_config_item.clone();
    cloned_tile_config_item.tiles = vec![];
    cloned_tile_config_item.ascii = vec![];
    cloned_tile_config_item.sprite_height = Some(
      cloned_tile_config_item
        .sprite_height
        .unwrap_or(raw_tile_config.tile_info.get(0).unwrap_or(&default_tileset_info).height),
    );
    cloned_tile_config_item.sprite_width = Some(
      cloned_tile_config_item
        .sprite_width
        .unwrap_or(raw_tile_config.tile_info.get(0).unwrap_or(&default_tileset_info).width),
    );
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
  let config_with_cache = tileset_json::CDDATileSetConfigWithCache { textures, tile_data_index };
  config_with_cache
}

#[tauri::command]
pub fn read_mapgen_file(mapgen_file_path: &str) -> mapgen_json::CDDAMapgenWithCache {
  let mapgen_absolute_file_path = Path::join(&Path::join(&get_project_root().unwrap(), "../public"), mapgen_file_path);
  // read mapgen
  let mut raw_mapgen_file = File::open(mapgen_absolute_file_path).unwrap();
  let mut raw_mapgen_string = String::new();
  raw_mapgen_file.read_to_string(&mut raw_mapgen_string).unwrap();
  let raw_mapgen: mapgen_json::CDDAMapgenArray = serde_json::from_str(&raw_mapgen_string).unwrap();
  // read palette
  let mut raw_palette_file = File::open("../public/json/house_general_palette.json").unwrap();
  let mut raw_palette_string = String::new();
  raw_palette_file.read_to_string(&mut raw_palette_string).unwrap();
  let raw_palette: palette_json::CDDAPaletteArray = serde_json::from_str(&raw_palette_string).unwrap();

  // TODO: search for correct palette to use
  // TODO: merge mapgen palette and raw_palette
  let standard_domestic_palette = raw_palette.get(0).unwrap();

  let parsed_map: Vec<Vec<Vec<Vec<mapgen_json::ItemIDOrItemList>>>> = raw_mapgen
    .iter()
    .map(|mapgen| {
      mapgen
        .object
        .rows
        .iter()
        .map(|row| row.chars().map(|c| lookup_mapgen_char_in_palette(&c, standard_domestic_palette)).collect())
        .collect()
    })
    .collect();
  let mapgen_with_cache = mapgen_json::CDDAMapgenWithCache { raw_mapgen, parsed_map };
  mapgen_with_cache
}

fn lookup_mapgen_char_in_palette(character: &char, palette: &palette_json::CDDAPalette) -> Vec<mapgen_json::ItemIDOrItemList> {
  let char_string = character.to_string();
  let mut items_this_tile: Vec<mapgen_json::ItemIDOrItemList> = vec![];
  // each type may have some different logic, so we cannot abstract these

  // terrain
  let terrain_value_option = palette.mapping_object.terrain.get(&char_string);
  match terrain_value_option {
    Some(terrain_value) => match terrain_value {
      palette_json::CDDAPaletteTerrainValue::Id(id) => {
        items_this_tile.push(mapgen_json::ItemIDOrItemList::Id((mapgen_json::MapgenPaletteKeys::terrain, id.clone())));
      }
      Others => {} // palette_json::CDDAPaletteTerrainValue::RandomList(ids) => {
                   //   items_this_tile.push(mapgen_json::ItemIDOrItemList::Id((mapgen_json::MapgenPaletteKeys::terrain, id.clone())));
                   // }
    },
    None => {}
  };
  // furniture
  // let furniture_value_option = palette.furniture.get(&char_string);
  // matchfurniture_value_option {
  //   Some(furniture) => {
  //     let furniture_value_option = furniture.get(&char_string);
  //     match furniture_value_option {
  //       Some(furniture_value) => match furniture_value {
  //         palette_json::CDDAPaletteFurnitureValue::Id(id) => id.clone(),
  //       },
  //     }
  //   }
  // }
  items_this_tile
}
