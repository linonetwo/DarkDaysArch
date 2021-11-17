use data::common::CDDAStringArray;
use data::types::CDDADataCluster;
use data::types::{mod_info, CDDA_JSON_Array, CDDA_JSON};
use glob::glob;
use project_root::get_project_root;
use std::path::Path;
use std::{collections::BTreeMap, fs::canonicalize, fs::File};

use crate::parsers;
use crate::types;

#[tauri::command]
pub fn load_cdda_data_folder<'s>(state: tauri::State<'s, types::state::AppState>, data_folder_path: String) -> Result<String, String> {
  if state.0.lock().unwrap().knowledge_graph.check_mod_folder_loaded(&data_folder_path) {
    return Err(format!("{} already loaded", &data_folder_path));
  }
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
              CDDA_JSON::furniture(json) => {
                match &json
                  .ter_furn_common
                  .select_list
                  .id
                  .clone()
                  .unwrap_or_else(|| panic!("Furniture missing id:\n{} {}", mod_id, file_path))
                {
                  CDDAStringArray::Single(id) => {
                    // .0 means get first variant from the tuple enum AppState
                    state.0.lock().unwrap().knowledge_graph.insert(&CDDADataCluster {
                      id: id.clone(),
                      type_field: "furniture".into(),
                      path: file_path.clone(),
                      data: data::types::CDDA_JSON::furniture(json.clone()),
                    });
                  }
                  CDDAStringArray::Multiple(ids) => {
                    for id in ids.clone() {
                      state.0.lock().unwrap().knowledge_graph.insert(&CDDADataCluster {
                        id: id.clone(),
                        type_field: "furniture".into(),
                        path: file_path.clone(),
                        data: data::types::CDDA_JSON::furniture(json.clone()),
                      });
                    }
                  }
                }
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
  state.0.lock().unwrap().knowledge_graph.set_mod_folder_loaded(data_folder_path);
  Ok("Done".into())
  // state.knowledge_graph
}
