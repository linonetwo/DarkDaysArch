use serde_json;
use std::fs::File;
use std::io::Read;
#[path = "../src/types/palette.rs"]
mod mapgen_json;
#[path = "../src/types/furniture.rs"]
mod furn_json;

fn main() {
  // let map_file_path = "../src/store/models/house_general_palette.json";
  // let mut raw_map_file = File::open(map_file_path).unwrap();
  // let mut raw_map_string = String::new();
  // raw_map_file.read_to_string(&mut raw_map_string).unwrap();
  // let raw_map: mapgen_json::CDDAPaletteArray = serde_json::from_str(&raw_map_string).unwrap();

  // let jsontext = serde_json::to_string(&raw_map).unwrap();

  let furn_file_path = "../src/store/models/furniture-medical.json";
  let mut raw_furn_file = File::open(furn_file_path).unwrap();
  let mut raw_furn_string = String::new();
  raw_furn_file.read_to_string(&mut raw_furn_string).unwrap();
  let raw_furn: furn_json::CDDAFurnArray = serde_json::from_str(&raw_furn_string).unwrap();

  let jsontext: String = serde_json::to_string(&raw_furn).unwrap();

  // println!("{:?}", raw_map);
  // println!("{:?}", raw_furn);
  println!("{}",jsontext);
}
