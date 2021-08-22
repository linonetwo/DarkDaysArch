use serde;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type CDDAMapgenArray = Vec<CDDAMapgen>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgen {
  #[serde(rename = "type")]
  pub type_field: String,
  pub method: String,
  #[serde(rename = "om_terrain")]
  pub om_terrain: String,
  pub object: CDDAMapgenObject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
  pub terrain: BTreeMap<String, String>,
  /**
   * @example `{ "=": "f_magic_bench", "-": "f_alembic", "?": "f_rack_wood" }`
   */
  pub furniture: Option<BTreeMap<String, String>>,
  #[serde(rename = "place_loot")]
  #[serde(default)]
  pub place_loot: Vec<CDDAMapgenPlaceLoot>,
  pub items: Option<BTreeMap<String, CDDAMapgenItemsArray>>,
  /**
   * @example `{ "=": "tr_rollmat" }`
   */
  pub traps: Option<BTreeMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenPlaceMonster {
  pub monster: String,
  pub x: i64,
  pub y: i64,
  pub density: f64,
  pub repeat: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenPlaceLoot {
  pub item: String,
  pub x: i64,
  pub y: i64,
  pub chance: i64,
}

pub type CDDAMapgenItemsArray = Vec<CDDAMapgenItems>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenItems {
  pub item: String,
  pub chance: i64,
  pub repeat: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenTraps {
  #[serde(rename = "=")]
  pub field: String,
}
