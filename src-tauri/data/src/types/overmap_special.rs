use crate::common::*;
use crate::list::SelectList;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

pub type CDDAOvermapSpecialArray = Vec<CDDAOvermapSpecial>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapSpecial {
  //no name allowed
  #[serde(flatten)]
  pub select_list: SelectList,

  #[serde(default)]
  #[serde(rename = "//")]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub comment: String,

  pub overmaps: Vec<CDDAOvermapSpecialOvermap>,
  /**
   * @docs OVERMAP  List of `overmap_location` ids that the special may be placed on.      
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub locations: Vec<String>,
  /**
   * @docs OVERMAP   Whether the special can rotate. True if not specified.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_false")]
  pub rotate: bool,
  /**
   * @docs OVERMAP and FLAG for overmap special
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub flags: Vec<CDDAOvermapSpecialFlag>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub spawns: Option<CDDAOvermapSpecialSpawns>,
  // overmap_special unique
  pub occurrences: (i64, i64),
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub connections: Vec<CDDAOvermapSpecialConnection>,
  /**
   * @docs OVERMAP Min/max distance from a city edge that the special may be placed. Use -1 for unbounded.  default (0,-1)
   */
  #[serde(default = "tuple::default_i_0m1")]
  #[serde(skip_serializing_if = "tuple::is_default_i_0m1")]
  pub city_distance: (i64, i64),
  /**
   * @docs OVERMAP Min/max city size for a city that the special may be placed near. Use -1 for unbounded.  default (0,-1)
   */
  #[serde(default = "tuple::default_i_0m1")]
  #[serde(skip_serializing_if = "tuple::is_default_i_0m1")]
  pub city_sizes: (i64, i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapSpecialConnection {
  /**
   * @docs OVERMAP  `[ x, y, z]` of the connection end point. Cannot overlap an overmap terrain entry for the special.
   * if do so, terrain can be omitted
   */
  pub point: (i64, i64, i64),
  /**
   * @docs OVERMAP  Will go away in favor of `connection` eventually. Use `road`, `subway`, `sewer`, etc.    overmap_terrain id
   */
  pub terrain: Option<String>,
  /**
   * @docs OVERMAP  Id of the `overmap_connection` to build. Optional for now, but you should specify it explicitly.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub connection: String,
  /**
   * @docs OVERMAP  Optional point `[ x, y, z]` within the special to treat as the origin of the connection.   undefined means no inner connection
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<(i64, i64, i64)>,
  /**
   * @docs OVERMAP  Boolean, default false. If the special requires a preexisting terrain to spawn.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_false")]
  pub existing: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapSpecialOvermap {
  /**
   * @docs OVERMAP  `[ x, y, z]` of the overmap terrain within the special.
   */
  pub point: (i64, i64, i64),
  /**
   * @docs OVERMAP  Id of the `overmap_terrain` to place at the location.  
   */
  pub overmap: String,
  /**
   * @docs OVERMAP  List of `overmap_location` ids that this overmap terrain may be placed on.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub locations: Vec<String>,
  /**
   * @srcs overmap
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub flags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapSpecialSpawns {
  /**
   * @src overmap  monster group id
   */
  #[serde(default)]
  #[serde(rename = "//")]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub group: String,
  /**
   * @src overmap  common_types  must be an interval   default 0 (maybe)
   */
  #[serde(default = "tuple::default_i_00")]
  #[serde(skip_serializing_if = "tuple::is_default_i_00")]
  pub population: (i64, i64),
  /**
   * @srcs radius from center?
   */
  #[serde(default = "tuple::default_i_00")]
  #[serde(skip_serializing_if = "tuple::is_default_i_00")]
  pub radius: (i64, i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum CDDAOvermapSpecialFlag {
  #[serde(rename = "ANT")]
  Ant,
  #[serde(rename = "BEE")]
  Bee,
  #[serde(rename = "BLOB")]
  Blob,
  #[serde(rename = "CLASSIC")]
  Classic,
  #[serde(rename = "FARM")]
  Farm,
  #[serde(rename = "FUNGAL")]
  Fungal,
  #[serde(rename = "LAB")]
  Lab,
  #[serde(rename = "LAKE")]
  Lake,
  #[serde(rename = "MI-GO")]
  MiGo,
  #[serde(rename = "MILITARY")]
  Military,
  #[serde(rename = "SAFE_AT_WORLDGEN")]
  SafeAtWorldgen,
  #[serde(rename = "SLIME")]
  Slime,
  #[serde(rename = "TRIFFID")]
  Triffid,
  #[serde(rename = "UNIQUE")]
  Unique,
  #[serde(rename = "URBAN")]
  Urban,
  #[serde(rename = "WILDERNESS")]
  Wilderness,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum Type {
  #[serde(rename = "overmap_special")]
  OvermapSpecial,
}
