use super::palette::*;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenWithCache {
  /**
   * Full mapgen file content, for code editor to display
   */
  pub raw_mapgen: CDDAMapgenArray,
  /**
   * Map 2D array that have place-holder characters replaced with actual item ID, for map view to display
   * And we have multiple mapgen in a file, so this will be a 3D matrix.
   * But each location can have terrain, furniture, item and so on, so each tile will be a list, so this is a 4D tensor
   */
  pub parsed_map: Vec<Vec<Vec<Vec<ItemIDOrItemList>>>>,
}

/**
 * A char in map rows can mean multiple item, like # mean a terrain and a furniture, and some terrain can have id same as a furniture, so we have to keep id's type in a tuple
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ItemIDOrItemList {
  /**
   * (type, id), where type is like "terrain" or "furniture"
   */
  Id((MapgenPaletteKeys, String)),
  ItemList(Vec<(MapgenPaletteKeys, String)>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum MapgenPaletteKeys {
  #[allow(non_camel_case_types)]
  terrain,
  #[allow(non_camel_case_types)]
  furniture,
}

pub type CDDAMapgenArray = Vec<CDDAMapgen>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgen {
  Om(CDDAMapgenOM),
  Update(CDDAMapgenUpdate),
  Nested(CDDAMapgenNested),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenCommon {
  #[serde(rename = "type")]
  pub type_field: String,
  #[serde(default)]
  pub method: String,
  pub object: Option<CDDAMapgenObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenOM {
  pub om_terrain: String,
  #[serde(flatten)]
  pub common: CDDAMapgenCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenUpdate {
  pub update_mapgen_id: String,
  #[serde(flatten)]
  pub common: CDDAMapgenCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenNested {
  pub nested_mapgen_id: String,
  #[serde(flatten)]
  pub common: CDDAMapgenCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenObject {
  pub fill_ter: String,
  pub rows: Vec<String>,
  #[serde(default)]
  pub palettes: Vec<String>,
  #[serde(flatten)]
  pub mapping_object: CDDAMapgenMapping,

  #[serde(default)]
  pub place_monsters: Vec<CDDAMapgenPlaceMonster>,
  // /**
  //  * @example `{ "_": "t_open_air", ")": "t_wall_glass", "#": "t_rock_wall", "-": "t_floor", "]": "t_door_glass_c" }`
  //  */
  // pub terrain: BTreeMap<String, CDDAMapgenTerrain>,
  // /**
  //  * @example `{ "=": "f_magic_bench", "-": "f_alembic", "?": "f_rack_wood" }`
  //  */
  // #[serde(default)]
  // pub furniture: BTreeMap<String, CDDAMapgenFurniture>,
  #[serde(default)]
  pub place_loot: Vec<CDDAMapgenPlaceLoot>,
  // #[serde(default)]
  // pub items: BTreeMap<String, CDDAMapgenItem>,
  // /**
  //  * @example `{ "=": "tr_rollmat" }`
  //  */
  // #[serde(default)]
  // pub traps: BTreeMap<String, CDDAMapgenTrap>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenTerrain {
  Id(String),
  RandomList(Vec<CDDAMapgenTerrainRandomListItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenTerrainRandomListItem {
  Id(String),
  RandomList((String, i32)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceMonster {
  pub monster: String,
  pub x: i64,
  pub y: i64,
  pub density: f64,
  pub repeat: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceLoot {
  pub item: String,
  pub x: i64,
  pub y: i64,
  pub chance: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenItem {
  Id(String),
  Item(CDDAMapgenItemRandomListItem),
  RandomList(Vec<CDDAMapgenItemRandomListItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenFurniture {
  Id(String),
  RandomList(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenTrap {
  Id(String),
  Trap(CDDAMapgenTrapObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenItemRandomListItem {
  pub item: String,
  pub chance: i64,
  #[serde(default)]
  pub repeat: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenTrapObject {
  pub trap: String,
}
