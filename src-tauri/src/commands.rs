use data::types::{furniture, mapgen, palette, terrain, tileset};
use glob::glob;
use image_base64::to_base64;
use project_root::get_project_root;
use std::path::Path;
use std::{collections::BTreeMap, fs::File, io::Read};

use crate::parsers;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder, read_mapgen_file, read_terrain_file]
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

  let tile_data_index = parsers::tileset::prepare_tile_data_index(raw_tile_config);
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

  // TODO: merge mapgen palette and raw_palette
  let standard_domestic_palette = raw_palette.get(0).unwrap();

  let parsed_map: Vec<Vec<Vec<Vec<mapgen::ItemIDOrItemList>>>> = raw_mapgen
    .iter()
    .map(|mapgen| match mapgen {
      mapgen::CDDAMapgen::Om(overmap_terrain_mapgen) => {
        let om_object_option = &overmap_terrain_mapgen.common.object;
        match om_object_option {
          Some(om_object) => {
            let palettes_to_reuse: Vec<palette::CDDAPalette> = om_object
              .palettes
              .iter()
              .map(|palette_id| {
                // TODO: search in the knowledge graph
                raw_palette.iter().find(|p| p.id == *palette_id)
              })
              .filter(|p| p.is_some())
              .map(|p| p.unwrap().clone())
              .collect();
            if palettes_to_reuse.len() == 0 {
              return vec![];
            }
            let merged_palette = parsers::palette::merge_palette_for_mapgen(&palettes_to_reuse);

            om_object
              .rows
              .iter()
              .map(|row| {
                row
                  .chars()
                  .map(|c| {
                    let char_string = c.to_string();
                    let mut tile_ids = parsers::palette::lookup_mapgen_char_in_palette(&char_string, merged_palette);
                    if !tile_ids.iter().any(|tile_id| match tile_id {
                      mapgen::ItemIDOrItemList::Id((tile_id_type, _)) => *tile_id_type == mapgen::MapgenPaletteKeys::terrain,
                      mapgen::ItemIDOrItemList::ItemList(item) => false,
                    }) {
                      tile_ids.insert(
                        0,
                        mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, om_object.fill_ter.clone())),
                      );
                    }
                    // if char_string == "H" {
                    //   println!("{:?}", tile_ids);
                    // }
                    tile_ids
                  })
                  .collect()
              })
              .collect()
          }
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

#[tauri::command]
pub fn read_terrain_file(terrain_file_path: &str) -> terrain::CDDATerrainArray {
  let terrain_absolute_file_path = Path::join(&Path::join(&get_project_root().unwrap(), "../public"), terrain_file_path);
  // read terrain
  let mut raw_terrain_file = File::open(terrain_absolute_file_path).unwrap();
  let mut raw_terrain_string = String::new();
  raw_terrain_file.read_to_string(&mut raw_terrain_string).unwrap();
  let raw_terrain: terrain::CDDATerrainArray = serde_json::from_str(&raw_terrain_string).unwrap();
  raw_terrain
}
