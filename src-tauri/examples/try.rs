use serde_json;
use serde_json::{Result,Value};
use std::{fs::File, io::Read};
use data::types::*;
use data::list;
use serde;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

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

  let oms_file_path = "../public/json/specials.json";
  let mut raw_oms_file = File::open(oms_file_path).unwrap();
  let mut raw_oms_string = String::new();
  raw_oms_file.read_to_string(&mut raw_oms_string).unwrap();
  let raw_oms: overmap_special::CDDAOvermapSpecialArray = serde_json::from_str(&raw_oms_string).unwrap();

  let oms_json = serde_json::to_string(&raw_oms).unwrap();

  // println!("{:?}", raw_oms);
  println!("{}",oms_json);

}
