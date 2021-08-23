use serde_json;
use std::fs::File;
use std::io::Read;
#[path = "../src/types/palette.rs"]
mod mapgen_json;

fn main() {
  let map_file_path = "/Users/lindongwu/Desktop/github/DarkDaysArch/src/store/models/house_general_palette.json";
  let mut raw_map_file = File::open(map_file_path).unwrap();
  let mut raw_map_string = String::new();
  raw_map_file.read_to_string(&mut raw_map_string).unwrap();
  let raw_map: mapgen_json::CDDAPaletteArray = serde_json::from_str(&raw_map_string).unwrap();

  println!("{:?}", raw_map);
}
