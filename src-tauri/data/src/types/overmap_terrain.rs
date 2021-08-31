use crate::common::*;
use crate::list::SelectList;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

pub type CDDAOvermapTerrainArray = Vec<CDDAOvermapTerrain>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapTerrain {
  #[serde(flatten)]
  pub select_list: SelectList,

  #[serde(rename = "copy-from")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub copy_from: Option<String>,
  //enum
  /**
   * @src overmap  default c_black
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub color: Option<String>,
  // can use acsii id
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sym: Option<String>,
  // flagd used by Overmap terrains in FLAG.md
  #[serde(skip_serializing_if = "Option::is_none")]
  pub flags: Option<Vec<String>>,
  /**
   * @docs OVERMAP  Affects player vision on overmap. Higher values obstruct vision more.  default 0
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub see_cost: Option<i64>,
  /**
   * @docs OVERMAP  Affects pathfinding cost. Higher values are harder to travel through   default 5
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub travel_cost: Option<i64>,
  /**
   * @docs OVERMAP Summed with values for adjacent overmap terrains to influence density of monsters spawned here  default 0
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mondensity: Option<i64>,
  /**
   * @docs OVERMAP Reference to a named `map_extras`, defines which map extras can be applied  default "none"
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extras: Option<String>,
  /**
   * @docs OVERMAP    Spawns added once at mapgen. Monster group, % chance, population range (min/max)
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub spawns: Option<CDDAOvermapTerrainSpawns>,
  /**
   * @docs OVERMAP    pecify a C++ mapgen function. Don't do this--use JSON.
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapgen: Option<Vec<CDDAOvermapTerrainMapgen>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapgen_straight: Option<Vec<CDDAOvermapTerrainMapgen>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapgen_curved: Option<Vec<CDDAOvermapTerrainMapgen>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapgen_end: Option<Vec<CDDAOvermapTerrainMapgen>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapgen_tee: Option<Vec<CDDAOvermapTerrainMapgen>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mapgen_four_way: Option<Vec<CDDAOvermapTerrainMapgen>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub delete: Option<CDDACopyChange>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extend: Option<CDDACopyChange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapTerrainMapgen {
  pub method: CDDAOvermapTerrainMapgenMethod,
  /**
   * @src overmap  only when method is builtin
   */
  pub name: Option<String>,
  /**
   * @src overmap  only when method is json and mey be a CDDAMapgen?
   */
  pub object: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum CDDAOvermapTerrainMapgenMethod {
  #[serde(rename = "builtin")]
  Builtin,
  #[serde(rename = "json")]
  Json,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapTerrainSpawns {
  /**
   * @src overmap  monster group id
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub group: String,
  /**
   * @src overmap  common_types  must be an interval   default 0 (maybe)
   */
  #[serde(default = "tuple::default_i_0m1")]
  #[serde(skip_serializing_if = "tuple::is_default_i_0m1")]
  pub population: (i64, i64),
  /**
   * @src overmap  i64   default 0
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub chance: i64,
}
