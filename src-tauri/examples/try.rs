use std::fs::File;
use std::io::Read;
use serde_json;
#[path = "../src/types/mapgen.rs"]
mod mapgen_json;

fn main() {
  let map_file_path = "/Users/linonetwo/Desktop/repo/DarkDaysArch/src/store/models/magic_academy.json";
  let mut raw_map_file = File::open(map_file_path).unwrap();
  let mut raw_map_string = String::new();
  raw_map_file.read_to_string(&mut raw_map_string).unwrap();
  let raw_map: mapgen_json::CDDAMapgenArray = serde_json::from_str(&raw_map_string).unwrap();

  println!("{:?}", raw_map);
}
