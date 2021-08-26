use serde_json;
use std::{fs::File, io::Read};
use data::types::{palette,furniture,mapgen,tileset};

fn main() {
  // let map_file_path = "../public/json/magic_academy.json";
  // let mut raw_map_file = File::open(map_file_path).unwrap();
  // let mut raw_map_string = String::new();
  // raw_map_file.read_to_string(&mut raw_map_string).unwrap();
  // let raw_map: mapgen::CDDAMapgenArray = serde_json::from_str(&raw_map_string).unwrap();

  // let map_json = serde_json::to_string_pretty(&raw_map).unwrap();

  // // println!("{:?}", raw_map);
  // println!("{}",map_json);

  let palette_file_path = "../public/json/house_general_palette.json";
  let mut raw_palette_file = File::open(palette_file_path).unwrap();
  let mut raw_palette_string = String::new();
  raw_palette_file.read_to_string(&mut raw_palette_string).unwrap();
  let raw_palette: palette::CDDAPaletteArray = serde_json::from_str(&raw_palette_string).unwrap();

  let palette_json = serde_json::to_string_pretty(&raw_palette).unwrap();

  println!("{:?}", raw_palette);
  // println!("{}", palette_json);

  // let furn_file_path = "../public/json/furniture-medical.json";
  // let mut raw_furn_file = File::open(furn_file_path).unwrap();
  // let mut raw_furn_string = String::new();
  // raw_furn_file.read_to_string(&mut raw_furn_string).unwrap();
  // let raw_furn: furniture::CDDAFurnArray = serde_json::from_str(&raw_furn_string).unwrap();

  // let furn_json = serde_json::to_string(&raw_furn).unwrap();

  // println!("{:?}", raw_furn);
  // println!("{}",furn_json);
}
