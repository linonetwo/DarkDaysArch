use serde;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use typescript_type_def::{TypeDef};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenWithCache {
  /**
   * Full mapgen file content, for code editor to display
   */
  pub raw_mapgen: CDDAMapgenArray,
  /**
   * Map 2D array that have place-holder characters replaced with actual item ID, for map view to display
   * And we have multiple mapgen in a file, so this will be a 3D array
   */
  pub parsed_map: Vec<Vec<Vec<String>>>,
}

pub type CDDAMapgenArray = Vec<CDDAMapgen>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgen {
  #[serde(rename = "type")]
  pub type_field: String,
  pub method: String,
  #[serde(rename = "om_terrain")]
  pub om_terrain: String,
  pub object: CDDAMapgenObject,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenObject {
  #[serde(rename = "fill_ter")]
  pub fill_ter: String,
  pub rows: Vec<String>,
  #[serde(default)]
  pub palettes: Vec<String>,
  #[serde(rename = "place_monsters")]
  #[serde(default)]
  pub place_monsters: Vec<CDDAMapgenPlaceMonster>,
  /**
   * @example `{ "_": "t_open_air", ")": "t_wall_glass", "#": "t_rock_wall", "-": "t_floor", "]": "t_door_glass_c" }`
   */
  pub terrain: BTreeMap<String, CDDAMapgenTerrain>,
  /**
   * @example `{ "=": "f_magic_bench", "-": "f_alembic", "?": "f_rack_wood" }`
   */
  pub furniture: Option<BTreeMap<String, CDDAMapgenFurniture>>,
  #[serde(rename = "place_loot")]
  #[serde(default)]
  pub place_loot: Vec<CDDAMapgenPlaceLoot>,
  pub items: Option<BTreeMap<String, CDDAMapgenItem>>,
  /**
   * @example `{ "=": "tr_rollmat" }`
   */
  pub traps: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAMapgenTerrain {
  Id(String),
  RandomList(Vec<CDDAMapgenTerrainRandomListItem>),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAMapgenTerrainRandomListItem {
  Id(String),
  RandomList((String, i32)),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenPlaceMonster {
  pub monster: String,
  pub x: i64,
  pub y: i64,
  pub density: f64,
  pub repeat: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenPlaceLoot {
  pub item: String,
  pub x: i64,
  pub y: i64,
  pub chance: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAMapgenItem {
  Id(String),
  Item(CDDAMapgenItemRandomListItem),
  RandomList(Vec<CDDAMapgenItemRandomListItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAMapgenFurniture {
  Id(String),
  RandomList(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenItemRandomListItem {
  pub item: String,
  pub chance: i64,
  pub repeat: Option<Vec<i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenTraps {
  #[serde(rename = "=")]
  pub field: String,
}
