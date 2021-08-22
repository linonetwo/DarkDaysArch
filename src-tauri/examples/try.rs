use dotenv;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Read;

#[path = "../src/types/tileset.rs"]
mod tileset_json;

fn main() {
  dotenv::from_filename(".env.local").ok();
  let config_file_path = env::var("CONFIG_FILE_PATH").unwrap();
  println!("{}", config_file_path);
  let mut raw_tile_config_file = File::open(config_file_path).unwrap();
  let mut raw_tile_config_string = String::new();
  raw_tile_config_file
    .read_to_string(&mut raw_tile_config_string)
    .unwrap();
  let raw_tile_config: tileset_json::CDDATileSetConfig =
    serde_json::from_str(&raw_tile_config_string).unwrap();
  let mut textures: BTreeMap<String, String> = BTreeMap::new();
  textures.insert("aaa".to_string(), "bbb".to_string());
  let raw_config = tileset_json::CDDATileSetConfigWithCache {
    raw_config: raw_tile_config,
    textures: textures,
  };
  println!("{:?}", raw_config);
}
