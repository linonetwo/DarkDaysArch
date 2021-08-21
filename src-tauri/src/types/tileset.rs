use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/**
 * Have all original JSON CDDATileSetConfig have, but with additional inverse index for fast look at things
 */
pub struct CDDATileSetConfigWithCache {
  #[serde(rename = "raw_config")]
  pub raw_config: CDDATileSetConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDATileSetConfig {
  #[serde(rename = "tile_info")]
  pub tile_info: Vec<CDDATileSetTileInfo>,
  #[serde(rename = "tiles-new")]
  pub tiles_new: Vec<CDDATileSetTilesNew>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDATileSetTileInfo {
  pub pixelscale: i64,
  pub width: i64,
  pub height: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDATileSetTilesNew {
  pub file: String,
  #[serde(rename = "//")]
  pub field: Option<String>,
  #[serde(rename = "sprite_width")]
  pub sprite_width: Option<i64>,
  #[serde(rename = "sprite_height")]
  pub sprite_height: Option<i64>,
  #[serde(rename = "sprite_offset_x")]
  pub sprite_offset_x: Option<i64>,
  #[serde(rename = "sprite_offset_y")]
  pub sprite_offset_y: Option<i64>,
  pub tiles: Vec<CDDATileSetTile>,
  #[serde(default)]
  pub ascii: Vec<CDDATileSetAscii>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDATileSetRandomSpriteDescItem {
  spirit: i64,
  weight: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CDDATileSetImageID {
  Id(i64),
  IdList(Vec<i64>),
  RandomList(Vec<CDDATileSetRandomSpriteDescItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CDDATileSetID {
  Id(String),
  IdList(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDATileSetTile {
  pub id: CDDATileSetID,
  pub fg: CDDATileSetImageID,
  pub rotates: Option<bool>,
  pub bg: CDDATileSetImageID,
  pub animated: Option<bool>,
  pub multitile: Option<bool>,
  #[serde(rename = "additional_tiles")]
  #[serde(default)]
  pub additional_tiles: Vec<CDDATileSetAdditionalTile>,
  #[serde(rename = "//")]
  pub field: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDATileSetAdditionalTile {
  pub id: String,
  pub fg: CDDATileSetImageID,
  pub bg: CDDATileSetImageID,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDATileSetAscii {
  pub offset: i64,
  pub bold: bool,
  pub color: String,
}
