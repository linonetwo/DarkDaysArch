use data::types::{furniture, mapgen, mod_info, palette, terrain, tileset, CDDA_JSON_Array, CDDA_JSON};
use glob::glob;
use image_base64::to_base64;
use project_root::get_project_root;
use std::path::Path;
use std::{collections::BTreeMap, fs::canonicalize, fs::File, io::Read};

use crate::parsers;
use crate::types;

pub fn invoke_handler() -> impl Fn(tauri::Invoke) + Send + Sync + 'static {
  tauri::generate_handler![read_tileset_folder, read_mapgen_file, read_terrain_file]
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
  Ok(mapgen_with_cache)
}

#[tauri::command]
pub fn read_terrain_file(terrain_file_path: &str) -> Result<terrain::CDDATerrainArray, String> {
  let terrain_absolute_file_path = Path::join(&Path::join(&get_project_root().map_err(|e| e.to_string())?, "../public"), terrain_file_path);
  // read terrain
  let mut raw_terrain_file = File::open(terrain_absolute_file_path).map_err(|e| e.to_string())?;
  let mut raw_terrain_string = String::new();
  raw_terrain_file.read_to_string(&mut raw_terrain_string).map_err(|e| e.to_string())?;
  let raw_terrain: terrain::CDDATerrainArray = serde_json::from_str(&raw_terrain_string).map_err(|e| e.to_string())?;
  Ok(raw_terrain)
}

// TODO: prepare a State that contains all JSON first
// #[tauri::command]
// pub fn get_json_by_id(id: String) -> Result<CDDA_JSON, String> {

// }

#[tauri::command]
pub fn load_cdda_data_folder(state: tauri::State<types::state::AppState>, data_folder_path: String) -> Result<String, String> {
  let data_folder_absolute_file_path: String = canonicalize(Path::join(
    &Path::join(&get_project_root().map_err(|e| e.to_string())?, "../public"),
    &data_folder_path,
  ))
  .map_err(|e| e.to_string())?
  .display()
  .to_string();
  // Map from mod id to mod fileNames
  let mut file_paths_in_folder: BTreeMap<String, Vec<std::string::String>> = BTreeMap::new();
  let mut mod_ids: Vec<String> = vec![];
  // search for all mod info, to get mod id
  for entry in glob(&format!("{}/*/modinfo.json", data_folder_absolute_file_path)).expect("Failed to read glob pattern in load_cdda_data_folder") {
    match entry {
      Ok(mod_info_file_absolute_path_buf) => {
        let mod_info_file_absolute_path: String = mod_info_file_absolute_path_buf.display().to_string();
        // read tile_config file
        let mod_info_json: mod_info::CDDAModInfoArray = serde_json::from_reader(File::open(mod_info_file_absolute_path_buf).map_err(|e| e.to_string())?)
          .map_err(|e| format!("{} {}", e.to_string(), mod_info_file_absolute_path))?;
        let mod_id = match mod_info_json
          .get(0)
          .unwrap_or_else(|| panic!("{} don't have a valid mod info.", mod_info_file_absolute_path))
        {
          mod_info::CDDAModInfoWithExternalOption::ModInfo(mod_info_item) => mod_info_item.id.clone(),
          _ => panic!("{} first item is not a mod info.", mod_info_file_absolute_path),
        };
        // create record in Map
        let mut file_paths_in_mod: Vec<std::string::String> = vec![];
        // get mod folder path
        let mut mod_folder_absolute_path = std::path::PathBuf::from(&mod_info_file_absolute_path);
        mod_folder_absolute_path.pop();
        // search for all files in this mod
        for entry in
          glob(&format!("{}/*/*.json", mod_folder_absolute_path.display().to_string())).expect("Failed to read 2nd glob pattern in load_cdda_data_folder")
        {
          match entry {
            Ok(mod_json_file_absolute_path_buf) => {
              file_paths_in_mod.push(mod_json_file_absolute_path_buf.display().to_string());
            }
            Err(e) => println!("{:?}", e),
          };
        }
        file_paths_in_folder.insert(mod_id.clone(), file_paths_in_mod);
        mod_ids.push(mod_id);
      }
      Err(e) => println!("{:?}", e),
    }
  }
  // reading Mods JSON
  for mod_id in mod_ids {
    for file_path in &file_paths_in_folder[&mod_id] {
      match serde_json::from_reader::<_, CDDA_JSON_Array>(File::open(file_path).map_err(|e| e.to_string())?) {
        Ok(json_array) => {
          for json_enum in json_array {
            match json_enum {
              CDDA_JSON::Furniture(json) => {
                println!("{} {:?} {:?} {}", mod_id, json.cdda_json_type, json.ter_furn_common.select_list.id, file_path);
              }
              Others => {}
            }
          }
        }
        Err(e) => {
          // println!("{:?}", &format!("Error when read JSON file {} {}", e.to_string(), file_path));
        }
      }
    }
  }
  Ok("".into())
  // state.knowledge_graph
}
