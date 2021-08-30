use serde_json;
use std::{fs::File, io::Read};
use data::types::{palette,furniture,mapgen,tileset,terrain};

fn main() {
  // let map_file_path = "../public/json/magic_academy.json";
  // let mut raw_map_file = File::open(map_file_path).unwrap();
  // let mut raw_map_string = String::new();
  // raw_map_file.read_to_string(&mut raw_map_string).unwrap();
  // let raw_map: mapgen::CDDAMapgenArray = serde_json::from_str(&raw_map_string).unwrap();

  // // let get_set = match raw_map.get(0).unwrap() {
  // //   mapgen::CDDAMapgen::Om(om) => {
  // //     match &om.common.object {
  // //       Some(object) => {
  // //         match object.set.get(0).unwrap() {
  // //           mapgen::CDDAMapgenSet::Point(b) => {
  // //             match b {
  // //               mapgen::CDDAMapgenSetPoint::terrain{id,coordinate,set_common} => {
  // //                 set_common.chance
  // //               }
  // //             }
  // //           }
  // //         }
  // //       }
  // //       None => {
  // //         1
  // //       }
  // //     }
  // //   }
  // //   mapgen::CDDAMapgen::Update(u) => {
  // //     1
  // //   }
  // //   mapgen::CDDAMapgen::Nested(n) => {
  // //     1
  // //   }
  // // };

  // let map_json = serde_json::to_string(&raw_map).unwrap();

  // println!("{:?}", raw_map);
  // // println!("{:?}", get_set);
  // // println!("{}",map_json);

  // let palette_file_path = "../public/json/house_general_palette.json";
  // let mut raw_palette_file = File::open(palette_file_path).unwrap();
  // let mut raw_palette_string = String::new();
  // raw_palette_file.read_to_string(&mut raw_palette_string).unwrap();
  // let raw_palette: palette::CDDAPaletteArray = serde_json::from_str(&raw_palette_string).unwrap();

  // let palette_json = serde_json::to_string(&raw_palette).unwrap();

  // println!("{:?}", raw_palette);
  // // println!("{}", palette_json);

  let furn_file_path = "../public/json/furniture.json";
  let mut raw_furn_file = File::open(furn_file_path).unwrap();
  let mut raw_furn_string = String::new();
  raw_furn_file.read_to_string(&mut raw_furn_string).unwrap();
  let raw_furn: furniture::CDDAFurnArray = serde_json::from_str(&raw_furn_string).unwrap();

  let furn_json = serde_json::to_string(&raw_furn).unwrap();

  // println!("{:?}", raw_furn);
  println!("{}",furn_json);

  let ter_file_path = "../public/json/terrain-floors-indoor.json";
  let mut raw_ter_file = File::open(ter_file_path).unwrap();
  let mut raw_ter_string = String::new();
  raw_ter_file.read_to_string(&mut raw_ter_string).unwrap();
  let raw_ter: terrain::CDDATerrainArray = serde_json::from_str(&raw_ter_string).unwrap();

  let ter_json = serde_json::to_string(&raw_ter).unwrap();

  // println!("{:?}", raw_ter);
  println!("{}",ter_json);
}
