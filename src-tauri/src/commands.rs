use data::types::{furniture, mapgen, palette, tileset};
use glob::glob;
use image_base64::to_base64;
use project_root::get_project_root;
use rand::Rng;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::{collections::BTreeMap, fs::File, io::Read};

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder, read_mapgen_file]
}

#[tauri::command]
pub fn read_tileset_folder(tileset_path_name: &str) -> tileset::CDDATileSetConfigWithCache {
  let tileset_absolute_file_path = Path::join(&Path::join(&get_project_root().unwrap(), "../public"), tileset_path_name)
    .into_os_string()
    .into_string()
    .unwrap();
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
  let raw_tile_config: tileset::CDDATileSetConfig = serde_json::from_str(&raw_tile_config_string).unwrap();
  // prepare textures
  let mut textures: BTreeMap<String, String> = BTreeMap::new();
  for tile_config_item in &raw_tile_config.tiles_new {
    let texture_image_file_path = format!("{}/{}", tileset_absolute_file_path, tile_config_item.file);
    textures.insert(tile_config_item.file.clone(), to_base64(&texture_image_file_path));
  }
  // prepare inverse index
  let default_tileset_info = tileset::CDDATileSetTileInfo {
    width: 32,
    height: 32,
    pixelscale: 1,
  };
  let id_start_matching_regex = Regex::new(r"range (\d+) to (\d+)").unwrap();
  let mut tile_data_index: BTreeMap<String, tileset::CDDATileSetInverseIndexedTileData> = BTreeMap::new();
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
        tileset::CDDATileSetID::Id(tile_id_string) => {
          tile_data_index.insert(
            tile_id_string.clone(),
            tileset::CDDATileSetInverseIndexedTileData {
              tile: tile_data_item.clone(),
              tileset: cloned_tile_config_item.clone(),
              start_id: id_start_matching_result,
            },
          );
        }
        tileset::CDDATileSetID::IdList(tile_id_list) => {
          for tile_id_string_in_list in tile_id_list {
            tile_data_index.insert(
              tile_id_string_in_list.clone(),
              tileset::CDDATileSetInverseIndexedTileData {
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
  let config_with_cache = tileset::CDDATileSetConfigWithCache { textures, tile_data_index };
  config_with_cache
}

#[tauri::command]
pub fn read_mapgen_file(mapgen_file_path: &str) -> mapgen::CDDAMapgenWithCache {
  let mapgen_absolute_file_path = Path::join(&Path::join(&get_project_root().unwrap(), "../public"), mapgen_file_path);
  // read mapgen
  let mut raw_mapgen_file = File::open(mapgen_absolute_file_path).unwrap();
  let mut raw_mapgen_string = String::new();
  raw_mapgen_file.read_to_string(&mut raw_mapgen_string).unwrap();
  let raw_mapgen: mapgen::CDDAMapgenArray = serde_json::from_str(&raw_mapgen_string).unwrap();
  // read palette
  let mut raw_palette_file = File::open("../public/json/house_general_palette.json").unwrap();
  let mut raw_palette_string = String::new();
  raw_palette_file.read_to_string(&mut raw_palette_string).unwrap();
  let raw_palette: palette::CDDAPaletteArray = serde_json::from_str(&raw_palette_string).unwrap();

  // TODO: search for correct palette to use
  // TODO: merge mapgen palette and raw_palette
  let standard_domestic_palette = raw_palette.get(0).unwrap();

  let parsed_map: Vec<Vec<Vec<Vec<mapgen::ItemIDOrItemList>>>> = raw_mapgen
    .iter()
    .map(|mapgen| match mapgen {
      mapgen::CDDAMapgen::Om(om) => {
        let object = &om.common.object;
        match object {
          Some(o) => o
            .rows
            .iter()
            .map(|row| row.chars().map(|c| lookup_mapgen_char_in_palette(&c, standard_domestic_palette)).collect())
            .collect(),
          None => vec![],
        }
      }
      // TODO: parse other type of mapgen
      Others => {
        vec![]
      }
    })
    .collect();
  let mapgen_with_cache = mapgen::CDDAMapgenWithCache { raw_mapgen, parsed_map };
  mapgen_with_cache
}

fn lookup_mapgen_char_in_palette(character: &char, palette: &palette::CDDAPalette) -> Vec<mapgen::ItemIDOrItemList> {
  let char_string = character.to_string();
  let mut items_this_tile: Vec<mapgen::ItemIDOrItemList> = vec![];
  // each type may have some different logic, so we cannot abstract these

  // terrain
  let terrain_value_option = palette.mapping_object.terrain.get(&char_string);
  match terrain_value_option {
    Some(terrain_value) => match terrain_value {
      // "a": "t_thconc_floor",
      palette::CDDAPaletteTerrainValue::Id(id) => {
        items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, id.clone())));
      }
      palette::CDDAPaletteTerrainValue::Object(terrain_value) => {
        items_this_tile.push(mapgen::ItemIDOrItemList::Id((
          mapgen::MapgenPaletteKeys::terrain,
          terrain_value.terrain.clone(),
        )));
      }
      // "o": [["t_window_domestic", 10], "t_window_no_curtains", "t_window_open", "t_window_no_curtains_open", ["t_curtains", 5]],
      // possible: [["t_window_domestic", 10], ["t_window_no_curtains", "t_window_open"], "t_window_no_curtains_open", [["t_curtains", 5], ["t_door_o", 5], "t_door_locked_interior"]
      palette::CDDAPaletteTerrainValue::RandomList(random_list_ids) => {
        let random_id = pick_random_list_id_by_distribution(&random_list_ids);
        match random_id {
          Some(id) => {
            items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, id)));
          }
          None => {}
        };
      }
      palette::CDDAPaletteTerrainValue::ParamRef(ref_object) => {
        let reference_parameter_option = &palette.mapping_object.parameters.get(&ref_object.param);
        match reference_parameter_option {
          Some(palette_parameter) => {
            let random_id = pick_random_list_id_by_distribution(&palette_parameter.default.distribution);
            match random_id {
              Some(id) => {
                items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, id)));
              }
              None => {}
            };
          }
          None => {
            items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, ref_object.fallback.clone())));
          }
        };
      }
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
  //         palette::CDDAPaletteFurnitureValue::Id(id) => id.clone(),
  //       },
  //     }
  //   }
  // }
  items_this_tile
}

struct RandomList<T: std::cmp::Eq + std::hash::Hash> {
  distribution: HashMap<T, i64>,
}
impl<T: std::cmp::Eq + std::hash::Hash> RandomList<T> {
  fn add(&mut self, item: T, frequency: i64) {
    self.distribution.insert(item, frequency);
  }

  /**
   * Get a random member from items you just added
   */
  fn random_get(&self) -> Option<&T> {
    let mut total_frequency = 0;
    for (item, frequency) in &self.distribution {
      total_frequency += frequency;
    }
    let mut rng = rand::thread_rng();
    let mut random_index = rng.gen_range(0..total_frequency);
    let mut last_item: Option<&T> = None;
    for (item, frequency) in &self.distribution {
      total_frequency -= frequency;
      if random_index >= total_frequency {
        return Some(item);
      }
      last_item = Some(item);
    }
    last_item
  }
}
impl<T: std::cmp::Eq + std::hash::Hash> Default for RandomList<T> {
  fn default() -> RandomList<T> {
    RandomList { distribution: HashMap::new() }
  }
}

fn pick_random_list_id_by_distribution(random_list_ids: &palette::CDDAPaletteDistribution) -> Option<String> {
  let mut random_list_item_picker = RandomList { ..Default::default() };
  match random_list_ids {
    palette::CDDAPaletteDistribution::Id(id) => {
      random_list_item_picker.add(id, 1);
    }
    palette::CDDAPaletteDistribution::IdList(id_list) => {
      for id in id_list {
        random_list_item_picker.add(id, 1);
      }
    }
    palette::CDDAPaletteDistribution::IdWithWeight(id, weight) => {
      random_list_item_picker.add(id, *weight);
    }
    palette::CDDAPaletteDistribution::RecursiveMixed(id_or_id_with_weight_list) => {
      for id_or_id_with_weight in id_or_id_with_weight_list {
        match id_or_id_with_weight {
          palette::CDDAPaletteDistributionMixed::Id(id) => {
            random_list_item_picker.add(id, 1);
          }
          palette::CDDAPaletteDistributionMixed::IdWithWeight(id, weight) => {
            random_list_item_picker.add(id, *weight);
          }
        }
      }
    }
  };
  // get random one from the list
  let result = random_list_item_picker.random_get();
  match result {
    Some(id) => Some((*id).clone()),
    None => None,
  }
}
