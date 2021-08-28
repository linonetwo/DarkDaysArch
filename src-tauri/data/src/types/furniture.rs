use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};
use crate::common::*;

pub type CDDAFurnArray = Vec<CDDAFurniture>;

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
// #[serde(untagged)]
// pub enum CDDAFurniture {
//   //type using color
//   Color(CDDAFurnitureCr),
//   //type using bgcolor
//   Background(CDDAFurnitureBg),
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnCommon {
  //mandatory
  #[serde(rename = "type")]
  pub type_field: String,

  pub id: String,

  pub name: CDDAName,

  pub description: String,

  #[serde(flatten)]
  pub symbol_clutter: CDDATerFurnSymbol,

  #[serde(flatten)]
  pub color_select: CDDATerFurnColorSelect,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub examine_action: String,

  #[serde(default)]
  pub coverage: i64,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub map_bash_info: String,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub map_deconstrunct_info: String,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub close: String,
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub open: String,
  /**
   * @srcs mapdata.cpp    furn to furn   ter to ter
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub curtain_transform: String,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_volume: Option<CDDAVolume>,

  #[serde(default)]
  pub flags: Vec<String>,

  #[serde(default)]
  pub connects_to: Vec<String>,

  #[serde(default)]
  pub comfort: i64,

  /**
   * @docs JSON_INFO.md    listing the `emit_id` of the fields the terrain/furniture will produce every 10 seconds
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub emissions: Vec<String>,

  #[serde(default)]
  pub floor_bedding_warmth: i64,

  #[serde(default)]
  pub bonus_fire_warmth_feet: i64,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub shoot: Option<CDDATerFurnShoot>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub boltcut: Option<CDDATerFurnBoltcut>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurniture {

  #[serde(flatten)]
  pub ter_furn_common: CDDATerFurnCommon,

  pub move_cost_mod: i64,

  pub required_str: i64,

  //optional

  //furniture unique key

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub crafting_pseudo_item: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub bash: Option<CDDAFurnBash>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub deconstruct: Option<CDDAFurnDecon>,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub workbench: Option<CDDAFurnWorkbench>,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plant_data: Option<CDDAFurnPlant>,

  #[serde(default)]
  pub surgery_skill_multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATerFurnSymbol {
  Independent(CDDATerFurnSymbolInd),
  Rely(CDDATerFurnSymbolRely)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnSymbolInd {
  pub symbol: String,
  #[serde(flatten)]
  pub color_select: CDDATerFurnColorSelect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnSymbolRely {
  pub looks_like: String,
  #[serde(default)]
  pub symbol: String,
  #[serde(default)]
  #[serde(flatten)]
  #[serde(skip_serializing_if = "CDDATerFurnColorSelect::is_default")]
  pub color_select: CDDATerFurnColorSelect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATerFurnColorSelect {
  Color{color: CDDATerFurnColor},
  Bgcolor{bgcolor: CDDATerFurnColor},
}

impl Default for CDDATerFurnColorSelect {
  fn default() -> CDDATerFurnColorSelect {
    CDDATerFurnColorSelect::Color{color: CDDATerFurnColor::Single("".to_string())}
  }
}

impl CDDATerFurnColorSelect {
  pub fn is_default(t: &CDDATerFurnColorSelect) -> bool {
    t == &CDDATerFurnColorSelect::default()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnWorkbench {
  pub multiplier: f64,
  pub mass: CDDAMass,
  pub volume: CDDAVolume,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnPlant {
  pub transform: String,
  pub base: String,

  pub growth_multiplier: f64,

  pub harvest_multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnFieldBashCommon {
  #[serde(default)]
  pub str_min: i64,

  #[serde(default)]
  pub str_max: i64,

  #[serde(default)]
  pub str_min_blocked: i64,

  #[serde(default)]
  pub str_max_blocked: i64,

  #[serde(default)]
  pub str_min_supported: i64,

  #[serde(default)]
  pub str_max_supported: i64,

  #[serde(default)]
  pub explosive: i64,

  #[serde(default)]
  pub sound_vol: i64,

  #[serde(default)]
  pub sound_fail_vol: i64,

  #[serde(default)]
  pub collapse_radius: i64,

  #[serde(default)]
  pub destroy_only: bool,

  #[serde(default)]
  pub bash_below: bool,

  #[serde(default)]
  pub sound: String,

  #[serde(default)]
  pub sound_fail: String,

  #[serde(default)]
  pub tent_centers: Vec<String>,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnBash {
  #[serde(flatten)]
  pub bash_common: CDDATerFurnFieldBashCommon,

  #[serde(default)]
  pub furn_set: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnDeconCommon {
  #[serde(default)]
  pub deconstruct_above: bool,

  #[serde(default)]
  pub can_do: bool,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnDecon {
  #[serde(flatten)]
  pub deconstruct_common: CDDATerFurnDeconCommon,

  #[serde(default)]
  pub furn_set: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDABashDeconItem {
  pub item: String,
  pub count: Option<CDDAIntRange>,
  pub charges: Option<CDDAIntRange>,
  #[serde(default)]
  pub prob: i64,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATerFurnColor {
  // one color
  Single(String),
  // four colors for four season, 2, 3 not sure
  Multiple(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnHarvest {
  /**
   * @docs JSON_INFO.md    in this seasons, item group with id can be got
   */
  pub seasons: Vec<String>,
  /**
   * @docs JSON_INFO.md    item group
   */
  pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnShoot {
  /**
   * @srcs mapdata.cpp  map_shoot_info
   */
  pub reduce_damage: (i64,i64),
  /**
   * @srcs mapdata.cpp  map_shoot_info
   */
  pub reduce_damage_laser: (i64,i64),
  /**
   * @srcs mapdata.cpp  map_shoot_info
   */
  pub destroy_damage: (i64,i64),
  /**
   * @srcs mapdata.cpp  map_shoot_info   default 100
   */
  #[serde(default)]
  pub chance_to_hit: i64,
  /**
   * @srcs mapdata.cpp  map_shoot_info   default false
   */
  #[serde(default)]
  pub no_laser_destroy: bool,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnBoltcut {
  /**
   * @srcs mapdata.cpp    activity_data_ter   ter or furn id    limited by original ter or furn
   */
  #[serde(default)]
  pub result: String,
  /**
   * @srcs mapdata.cpp    activity_data_common 
   */
  // #[serde(default)]
  pub duration: Option<CDDATime>,
  /**
   * @srcs mapdata.cpp    activity_data_common 
   */
  #[serde(default)]
  pub message: String,
  /**
   * @srcs mapdata.cpp    activity_data_common 
   */
  #[serde(default)]
  pub sound: String,
  /**
   * @srcs mapdata.cpp    activity_data_common 
   */
  #[serde(default)]
  pub valid: bool,
  /**
   * @srcs mapdata.cpp    activity_data_common 
   */
  #[serde(default)]
  pub byproducts: Vec<CDDAByproduct>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAByproduct {
  /**
   * @srcs mapdata.cpp    activity_byproduct
   */
  pub item: String,
  pub count: CDDAIntRange,
}