use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;

/**
 * Have all original JSON CDDATileSetConfig have, but with additional inverse index for fast look at things
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetConfigWithCache {
  /**
   * key is file name like `large.png`, value is base64 blob data string
   */
  pub textures: BTreeMap<String, String>,
  /**
   * inverse index to quick lookup tile data, key is tile name like `ranch_camp_17`, value is data I think useful for React renderer.
   */
  pub tile_data_index: BTreeMap<String, CDDATileSetInverseIndexedTileData>,
}

/**
 * value for tile_data_index
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetInverseIndexedTileData {
  /**
   * copy of tile data
   */
  pub tile: CDDATileSetTile,
  /**
   * copy of tileset image data, omit the `tiles` `ascii` field.
   */
  pub tileset: CDDATileSetTilesNew,
  /**
   * id in whole tileset is consequent, so each png 's tile 's id should have minus the start_id of this png
   */
  pub start_id: i64,
}

/**
 * Auto generated type from tile_config.json, enum are adjusted by hand.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetConfig {
  pub tile_info: Vec<CDDATileSetTileInfo>,

  pub tiles_new: Vec<CDDATileSetTilesNew>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetTileInfo {
  pub pixelscale: i64,
  pub width: i64,
  pub height: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetTilesNew {
  pub file: String,
  #[serde(rename = "//")]
  pub comment: Option<String>,

  pub sprite_width: Option<i64>,

  pub sprite_height: Option<i64>,

  pub sprite_offset_x: Option<i64>,

  pub sprite_offset_y: Option<i64>,
  pub tiles: Vec<CDDATileSetTile>,
  #[serde(default)]
  pub ascii: Vec<CDDATileSetAscii>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetRandomSpriteDescItem {
  sprite: i64,
  weight: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATileSetImageID {
  Id(i64),
  IdList(Vec<i64>),
  RandomList(Vec<CDDATileSetRandomSpriteDescItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATileSetID {
  Id(String),
  IdList(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetTile {
  pub id: CDDATileSetID,
  pub fg: Option<CDDATileSetImageID>,
  pub rotates: Option<bool>,
  pub bg: Option<CDDATileSetImageID>,
  pub animated: Option<bool>,
  pub multitile: Option<bool>,

  #[serde(default)]
  pub additional_tiles: Vec<CDDATileSetAdditionalTile>,
  #[serde(serialize = "//")]
  pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetAdditionalTile {
  pub id: String,
  pub fg: Option<CDDATileSetImageID>,
  pub bg: Option<CDDATileSetImageID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATileSetAscii {
  pub offset: i64,
  pub bold: bool,
  pub color: String,
}
