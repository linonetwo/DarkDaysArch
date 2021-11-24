pub mod load_cdda_data_folder;

use data::common::CDDAStringArray;
use data::types::{furniture, mapgen, mod_info, palette, terrain, tileset, CDDA_JSON_Array, CDDA_JSON};
use glob::glob;
use image_base64::to_base64;
use project_root::get_project_root;
use std::path::Path;
use std::{collections::BTreeMap, fs::canonicalize, fs::File, io::Read};

use crate::parsers;
use crate::types;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder, read_mapgen_file, load_cdda_data_folder::load_cdda_data_folder]
}

#[tauri::command]
pub fn read_tileset_folder(tileset_path_name: &str) -> Result<tileset::CDDATileSetConfigWithCache, String> {
  let tileset_absolute_file_path: String = Path::join(&Path::join(&get_project_root().map_err(|e| e.to_string())?, "../public"), tileset_path_name)
    .to_str()
    .ok_or_else(|| format!("Path join failed in read_tileset_folder({})", tileset_path_name))?
    .into();
  let mut files_in_folder: Vec<std::string::String> = vec![];
  for entry in glob(&format!("{}/*", tileset_absolute_file_path)).expect("Failed to read glob pattern") {
    match entry {
      Ok(path) => {
        files_in_folder.push(
          path
            .to_str()
            .ok_or_else(|| format!("glob failed in read_tileset_folder({})", tileset_path_name))?
            .into(),
        );
      }
      Err(e) => println!("{:?}", e),
    }
  }
  // read tile_config file
  let config_file_path = format!("{}/tile_config.json", tileset_absolute_file_path);
  let mut raw_tile_config_file = File::open(config_file_path).map_err(|e| e.to_string())?;
  let mut raw_tile_config_string = String::new();
  raw_tile_config_file.read_to_string(&mut raw_tile_config_string).map_err(|e| e.to_string())?;
  let raw_tile_config: tileset::CDDATileSetConfig = serde_json::from_str(&raw_tile_config_string).map_err(|e| e.to_string())?;
  // prepare textures
  let mut textures: BTreeMap<String, String> = BTreeMap::new();
  for tile_config_item in &raw_tile_config.tiles_new {
    let texture_image_file_path = format!("{}/{}", tileset_absolute_file_path, tile_config_item.file);
    textures.insert(tile_config_item.file.clone(), to_base64(&texture_image_file_path));
  }

  let tile_data_index = parsers::tileset::prepare_tile_data_index(raw_tile_config);
  let config_with_cache = tileset::CDDATileSetConfigWithCache { textures, tile_data_index };
  Ok(config_with_cache)
}

#[tauri::command]
pub fn read_mapgen_file(mapgen_file_path: &str) -> Result<mapgen::CDDAMapgenWithCache, String> {
  let mapgen_absolute_file_path = Path::join(&Path::join(&get_project_root().map_err(|e| e.to_string())?, "../public"), mapgen_file_path);
  // read mapgen
  let mut raw_mapgen_file = File::open(mapgen_absolute_file_path).map_err(|e| e.to_string())?;
  let mut raw_mapgen_string = String::new();
  raw_mapgen_file.read_to_string(&mut raw_mapgen_string).map_err(|e| e.to_string())?;
  let raw_mapgen: mapgen::CDDAMapgenArray = serde_json::from_str(&raw_mapgen_string).map_err(|e| e.to_string())?;
  // read palette
  let mut raw_palette_file = File::open("../public/json/house_general_palette.json").map_err(|e| e.to_string())?;
  let mut raw_palette_string = String::new();
  raw_palette_file.read_to_string(&mut raw_palette_string).map_err(|e| e.to_string())?;
  let raw_palette: palette::CDDAPaletteArray = serde_json::from_str(&raw_palette_string).map_err(|e| e.to_string())?;

  // TODO: merge mapgen palette and raw_palette
  let standard_domestic_palette = raw_palette
    .get(0)
    .ok_or_else(|| format!("raw_palette.get(0) failed in read_mapgen_file({})", mapgen_file_path))?;

  let parsed_map: Vec<Vec<Vec<Vec<mapgen::ItemId>>>> = raw_mapgen
    .iter()
    .map(|mapgen| match mapgen {
      mapgen::CDDAMapgen::Om(overmap_terrain_mapgen) => {
        let om_object_option = &overmap_terrain_mapgen.common.object;
        match om_object_option {
          Some(om_object) => {
            let mut palettes_to_reuse: Vec<palette::CDDAPalette> = om_object
              .palettes
              .iter()
              .map(|palette_id| {
                // TODO: search in the knowledge graph
                raw_palette.iter().find(|p| {
                  if let Some(id_mix) = &p.select_list.id {
                    match id_mix {
                      CDDAStringArray::Single(id) => *id == *palette_id,
                      CDDAStringArray::Multiple(ids) => {
                        let mut flag: bool = false;
                        for id in ids {
                          if *id == *palette_id {
                            flag = true;
                            break;
                          }
                        }
                        flag
                      }
                    }
                  } else {
                    false
                  }
                })
              })
              .filter(|p| p.is_some())
              .map(|p| p.unwrap().clone())
              .collect();
            // palette in the right will overwrite the one on the left, so we add mapgen palette to the right most position in this array
            palettes_to_reuse.push(parsers::palette::mapgen_to_palette(om_object));
            let merged_palette = parsers::palette::merge_palette_for_mapgen(&mut palettes_to_reuse);

            om_object
              .rows
              .iter()
              .map(|row| {
                row
                  .chars()
                  .map(|c| {
                    let char_string = c.to_string();
                    let mut tile_ids = parsers::palette::lookup_mapgen_char_in_palette(&char_string, &merged_palette);
                    if !tile_ids.iter().any(|item| item.0 == mapgen::MapgenPaletteKeys::terrain) {
                      tile_ids.insert(0, mapgen::ItemId(mapgen::MapgenPaletteKeys::terrain, om_object.fill_ter.clone()));
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
  Ok(mapgen_with_cache)
}

#[tauri::command]
pub fn query_database<'s>(state: tauri::State<'s, types::state::AppState>, sql: String) -> Result<Vec<CDDA_JSON>, String> {
  let database = &state.0.lock().unwrap().knowledge_graph.database;
  let mut stmt = database.prepare(&sql).map_err(|e| e.to_string())?;
  let results = stmt.query_map::<Vec<CDDA_JSON>, _, _>([], |r| r.get(0)).map_err(|e| e.to_string())?;
  Ok(results.collect::<Vec<CDDA_JSON>>())
}
