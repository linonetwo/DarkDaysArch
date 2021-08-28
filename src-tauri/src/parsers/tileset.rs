use regex::Regex;
use std::{collections::BTreeMap, fs::File, io::Read};

use data::types::tileset;

pub fn prepare_tile_data_index(raw_tile_config: tileset::CDDATileSetConfig) -> BTreeMap<String, tileset::CDDATileSetInverseIndexedTileData> {
  let id_start_matching_regex = Regex::new(r"range (\d+) to (\d+)").unwrap();
  let mut tile_data_index: BTreeMap<String, tileset::CDDATileSetInverseIndexedTileData> = BTreeMap::new();
  // prepare inverse index
  let default_tileset_info = tileset::CDDATileSetTileInfo {
    width: 32,
    height: 32,
    pixelscale: 1,
  };
  let tileset_info = raw_tile_config.tile_info.get(0).unwrap_or(&default_tileset_info);
  for tile_config_item in &raw_tile_config.tiles_new {
    let mut cloned_tile_config_item = tile_config_item.clone();
    cloned_tile_config_item.tiles = vec![];
    cloned_tile_config_item.ascii = vec![];
    cloned_tile_config_item.sprite_height = Some(cloned_tile_config_item.sprite_height.unwrap_or(tileset_info.height));
    cloned_tile_config_item.sprite_width = Some(cloned_tile_config_item.sprite_width.unwrap_or(tileset_info.width));
    // we store ratio of width height of tile to the tileset config width height, and we multiply this ratio to the visual width height defined in the render side
    cloned_tile_config_item.sprite_height_ratio = Some(cloned_tile_config_item.sprite_height.unwrap() / tileset_info.height);
    cloned_tile_config_item.sprite_width_ratio = Some(cloned_tile_config_item.sprite_width.unwrap() / tileset_info.width);
    cloned_tile_config_item.sprite_offset_x = Some(cloned_tile_config_item.sprite_offset_x.unwrap_or(0));
    cloned_tile_config_item.sprite_offset_y = Some(cloned_tile_config_item.sprite_offset_y.unwrap_or(0));
    cloned_tile_config_item.comment = Some(cloned_tile_config_item.comment.unwrap_or(String::from("range 1 to 63")));
    let id_start_matching_result: i64 = id_start_matching_regex
      .captures(&cloned_tile_config_item.comment.clone().unwrap())
      .unwrap()
      .get(1)
      .unwrap()
      .as_str()
      .parse::<i64>()
      .unwrap();
    for tile_data_item in &tile_config_item.tiles {
      match &tile_data_item.id {
        tileset::CDDATileSetID::Id(tile_id_string) => {
          tile_data_index.insert(
            tile_id_string.clone(),
            tileset::CDDATileSetInverseIndexedTileData {
              tile: tile_data_item.clone(),
              tileset: cloned_tile_config_item.clone(),
              start_id: id_start_matching_result,
            },
          );
        }
        tileset::CDDATileSetID::IdList(tile_id_list) => {
          for tile_id_string_in_list in tile_id_list {
            tile_data_index.insert(
              tile_id_string_in_list.clone(),
              tileset::CDDATileSetInverseIndexedTileData {
                tile: tile_data_item.clone(),
                tileset: cloned_tile_config_item.clone(),
                start_id: id_start_matching_result,
              },
            );
          }
        }
      }
    }
  }
  tile_data_index
}
