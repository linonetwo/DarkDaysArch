use data::list;
use data::types::*;
use glob::glob;
use project_root::get_project_root;
use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;
use std::{collections::BTreeMap, fs::canonicalize, fs::File, io::Read};


fn main() {
  // let map_file_path = "../public/json/magic_academy.json";
  // let mut raw_map_file = File::open(map_file_path).unwrap();
  // let mut raw_map_string = String::new();
  // raw_map_file.read_to_string(&mut raw_map_string).unwrap();
  // let raw_map: mapgen::CDDAMapgenArray = serde_json::from_str(&raw_map_string).unwrap();

  // let map_json = serde_json::to_string(&raw_map).unwrap();

  // println!("{:?}", raw_map);
  // println!("{}",map_json);

  // let palette_file_path = "../public/json/house_general_palette.json";
  // let mut raw_palette_file = File::open(palette_file_path).unwrap();
  // let mut raw_palette_string = String::new();
  // raw_palette_file.read_to_string(&mut raw_palette_string).unwrap();
  // let raw_palette: palette::CDDAPaletteArray = serde_json::from_str(&raw_palette_string).unwrap();

  // let palette_json = serde_json::to_string(&raw_palette).unwrap();

  // println!("{:?}", raw_palette);
  // // println!("{}", palette_json);

  // let furn_file_path = "../public/json/furniture.json";
  // let mut raw_furn_file = File::open(furn_file_path).unwrap();
  // let mut raw_furn_string = String::new();
  // raw_furn_file.read_to_string(&mut raw_furn_string).unwrap();
  // let raw_furn: furniture::CDDAFurnArray = serde_json::from_str(&raw_furn_string).unwrap();

  // let furn_json = serde_json::to_string(&raw_furn).unwrap();

  // // println!("{:?}", raw_furn);
  // println!("{}",furn_json);

  // let ter_file_path = "../public/json/terrain-floors-indoor.json";
  // let mut raw_ter_file = File::open(ter_file_path).unwrap();
  // let mut raw_ter_string = String::new();
  // raw_ter_file.read_to_string(&mut raw_ter_string).unwrap();
  // let raw_ter: terrain::CDDATerrainArray = serde_json::from_str(&raw_ter_string).unwrap();

  // let ter_json = serde_json::to_string(&raw_ter).unwrap();

  // // println!("{:?}", raw_ter);
  // println!("{}",ter_json);

  // let list_file_path = "../public/json/terrain-floors-indoor.json";
  // let mut raw_list_file = File::open(list_file_path).unwrap();
  // let mut raw_list_string = String::new();
  // raw_list_file.read_to_string(&mut raw_list_string).unwrap();
  // let raw_list: list::SelectListArray = serde_json::from_str(&raw_list_string).unwrap();

  // let list_json = serde_json::to_string(&raw_list).unwrap();

  // println!("{:?}", raw_list);
  // println!("{}",list_json);

  // let omt_file_path = "../public/json/overmap_terrain_commercial.json";
  // let mut raw_omt_file = File::open(omt_file_path).unwrap();
  // let mut raw_omt_string = String::new();
  // raw_omt_file.read_to_string(&mut raw_omt_string).unwrap();
  // let raw_omt: overmap_terrain::CDDAOvermapTerrainArray = serde_json::from_str(&raw_omt_string).unwrap();

  // let omt_json = serde_json::to_string(&raw_omt).unwrap();

  // // println!("{:?}", raw_omt);
  // println!("{}",omt_json);

  // let oms_file_path = "../public/json/specials.json";
  // let mut raw_oms_file = File::open(oms_file_path).unwrap();
  // let mut raw_oms_string = String::new();
  // raw_oms_file.read_to_string(&mut raw_oms_string).unwrap();
  // let raw_oms: overmap_special::CDDAOvermapSpecialArray = serde_json::from_str(&raw_oms_string).unwrap();

  // let oms_json = serde_json::to_string(&raw_oms).unwrap();

  // println!("{:?}", raw_oms);
  // println!("{}",oms_json);


  // let region_file_path = "../public/json/regional_map_settings.json";
  // let mut raw_region_file = File::open(region_file_path).unwrap();
  // let mut raw_region_string = String::new();
  // raw_region_file.read_to_string(&mut raw_region_string).unwrap();
  // let raw_region: region_settings::CDDARegionSettingsArray = serde_json::from_str(&raw_region_string).unwrap();

  // let region_json = serde_json::to_string(&raw_region).unwrap();

  // // println!("{:?}", raw_region);
  // println!("{}",region_json);

  // let city_file_path = "../public/json/city_buildings.json";
  // let mut raw_city_file = File::open(city_file_path).unwrap();
  // let mut raw_city_string = String::new();
  // raw_city_file.read_to_string(&mut raw_city_string).unwrap();
  // let raw_city: city_building::CDDACityBuildingArray = serde_json::from_str(&raw_city_string).unwrap();

  // let city_json = serde_json::to_string(&raw_city).unwrap();

  // // println!("{:?}", raw_city);
  // println!("{}",city_json);

  // let omconnection_file_path = "../public/json/overmap_connections.json";
  // let mut raw_omconnection_file = File::open(omconnection_file_path).unwrap();
  // let mut raw_omconnection_string = String::new();
  // raw_omconnection_file.read_to_string(&mut raw_omconnection_string).unwrap();
  // let raw_omconnection: overmap_connection::CDDAOvermapConnectionArray = serde_json::from_str(&raw_omconnection_string).unwrap();

  // let omconnection_json = serde_json::to_string(&raw_omconnection).unwrap();

  // // println!("{:?}", raw_omconnection);
  // println!("{}",omconnection_json);

  let oml_file_path = "../public/json/special_locations.json";
  let mut raw_oml_file = File::open(oml_file_path).unwrap();
  let mut raw_oml_string = String::new();
  raw_oml_file.read_to_string(&mut raw_oml_string).unwrap();
  let raw_oml: overmap_location::CDDAOvermapLocationArray = serde_json::from_str(&raw_oml_string).unwrap();

  let oml_json = serde_json::to_string(&raw_oml).unwrap();

  println!("{:?}", raw_oml);
  // println!("{}",oml_json);

  // let result = load_cdda_data_folder("assets/Kenan-Modpack-Chinese".into());
  // println!("{:?}", result);
}

pub fn load_cdda_data_folder(data_folder_path_name: String) -> Result<String, String> {
  let data_folder_absolute_file_path: String = canonicalize(Path::join(
    &Path::join(&get_project_root().map_err(|e| e.to_string())?, "../public"),
    &data_folder_path_name,
  ))
  .map_err(|e| e.to_string())?
  .display()
  .to_string();
  // Map from mod id to mod fileNames
  let mut files_in_folder: BTreeMap<String, Vec<std::string::String>> = BTreeMap::new();
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
        let mut file_names_in_mod: Vec<std::string::String> = vec![];
        // get mod folder path
        let mut mod_folder_absolute_path = std::path::PathBuf::from(&mod_info_file_absolute_path);
        mod_folder_absolute_path.pop();
        // search for all files in this mod
        for entry in glob(&format!("{}/*/*.json", mod_folder_absolute_path.display().to_string())).expect("Failed to read 2nd glob pattern in load_cdda_data_folder") {
          match entry {
            Ok(mod_json_file_absolute_path_buf) => {
              file_names_in_mod.push(mod_json_file_absolute_path_buf.display().to_string());
            }
            Err(e) => println!("{:?}", e),
          };
        }
        files_in_folder.insert(mod_id, file_names_in_mod);
      }
      Err(e) => println!("{:?}", e),
    }
  }
  println!("{:?}", files_in_folder);
  Ok("".into())
  // state.knowledge_graph
}
