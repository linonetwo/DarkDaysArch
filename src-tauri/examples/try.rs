use std::fs::File;
use std::io::Read;

#[path = "../src/types/tileset.rs"]
mod tileset_json;

fn main() {
  let mut raw_tile_config_file = File::open(
    "/Users/linonetwo/Desktop/repo/DarkDaysArch/public/assets/ChibiUltica/tile_config.json",
  )
  .unwrap();
  let mut raw_tile_config_string = String::new();
  raw_tile_config_file
    .read_to_string(&mut raw_tile_config_string)
    .unwrap();
  let raw_tile_config: tileset_json::CDDATileSetConfig =
    serde_json::from_str(&raw_tile_config_string).unwrap();
  let raw_config = tileset_json::CDDATileSetConfigWithCache {
    raw_config: raw_tile_config,
  };
}
