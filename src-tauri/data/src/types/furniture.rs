use crate::common::*;
use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};

pub type CDDAFurnArray = Vec<CDDAFurniture>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnCommon {
  //mandatory
  #[serde(rename = "type")]
  pub type_field: String,

  pub id: String,

  /**
   * fields about symbol including looks_like
   */
  #[serde(flatten)]
  pub symbol_clutter: CDDATerFurnSymbol,
  /**
   * @docs JSON_INFO.md   from an examine action list
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub examine_action: String,
  /**
   * @docs JSON_INFO.md   persentage of coverage
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub coverage: i64,
  /**
   * @docs JSON_INFO.md   transforms to when closed or opened
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub close: String,
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub open: String,
  /**
   * @srcs mapdata.cpp    furn needs furn id   ter needs ter id
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub curtain_transform: String,
  /**
   * @srcs mapdata.cpp    default is equal to DEFAULT_MAX_VOLUME_IN_SQUARE=1000L
   */
  #[serde(default = "CDDAVolume::default_1000l")]
  #[serde(skip_serializing_if = "CDDAVolume::is_default_1000l")]
  pub max_volume: CDDAVolume,
  /**
   * @docs JSON_INFO.md   flags
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub flags: Vec<String>,
  /**
   * @docs JSON_INFO.md   can connect to some special types defined by flags
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub connects_to: Vec<String>,
  /**
   * @docs JSON_INFO.md   comfort  How comfortable this terrain/furniture is. Impact ability to fall asleep on it.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub comfort: i64,
  /**
   * @docs JSON_INFO.md    listing the `emit_id` of the fields the terrain/furniture will produce every 10 seconds
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub emissions: Vec<String>,
  /**
   * @docs JSON_INFO.md   Bonus warmth offered by this terrain/furniture when used to sleep.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub floor_bedding_warmth: i64,
  /**
   * @docs JSON_INFO.md   Increase warmth received on feet from nearby fire  (default = 300)
   */
  #[serde(default = "int64::default_300")]
  #[serde(skip_serializing_if = "int64::is_default_300")]
  pub bonus_fire_warmth_feet: i64,
  /**
   * @srcs mapdata.cpp   Data for being shot    if not defined, cannot be shot to broken   0.F not included
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shoot: Option<CDDATerFurnShoot>,
  /**
   * @docs JSON_INFO.md  Data for using with an bolt cutter    if not defined, cannot be boltcut ?   0.F not included
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boltcut: Option<CDDATerFurnBoltcut>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAFurnitureOmittable {
  Optional(CDDAFurnitureOptional),
  Mandatory(CDDAFurnitureMandatory),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnitureMandatory {
  /**
   * this field have a default value CDDAName::Name(""), which need to be replaced with copied one
   */
  pub name: CDDAName,
  /**
   * this field have a default value "", which need to be replaced with copied one
   */
  pub description: String,
  /**
   * Movement cost modifier (`-10` = impassable, `0` = no change). This is added to the movecost of the underlying terrain.
   */
  pub move_cost_mod: i64,
  /**
   * Strength required to move the furniture around. Negative values indicate an unmovable furniture.
   */
  pub required_str: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnitureOptional {
  #[serde(rename = "copy-from")]
  pub copy_from: String,
  /**
   * this field have a default value CDDAName::Name(""), which need to be replaced with copied one
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAName::is_default")]
  pub name: CDDAName,
  /**
   * this field have a default value "", which need to be replaced with copied one
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub description: String,
  /**
   * Movement cost modifier (`-10` = impassable, `0` = no change). This is added to the movecost of the underlying terrain.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub move_cost_mod: i64,
  /**
   * Strength required to move the furniture around. Negative values indicate an unmovable furniture.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub required_str: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurniture {
  #[serde(flatten)]
  pub ter_furn_common: CDDATerFurnCommon,

  #[serde(flatten)]
  pub furn_omittable: CDDAFurnitureOmittable,
  /**
   * @docs JSON_INFO.md  Id of an item (tool) that will be available for crafting when this furniture is range
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub crafting_pseudo_item: String,
  /**
   * @docs JSON_INFO.md   if not defined, cannot be bashed to broken
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bash: Option<CDDAFurnBash>,
  /**
   * @docs JSON_INFO.md   if not defined, cannot be deconstructed
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deconstruct: Option<CDDAFurnDecon>,
  /**
   * @docs JSON_INFO.md   if not defined, cannot be a workbench
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub workbench: Option<CDDAFurnWorkbench>,
  /**
   * @docs JSON_INFO.md   if not defined, can not be planted with things
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plant_data: Option<CDDAFurnPlant>,
  /**
   * @docs JSON_INFO.md   default  null?1.0?   your surgery skill will be modified
   */
  #[serde(default = "float64::default_1")]
  #[serde(skip_serializing_if = "float64::is_default_1")]
  pub surgery_skill_multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATerFurnSymbol {
  Independent(CDDATerFurnSymbolInd),
  Rely(CDDATerFurnSymbolRely),
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
  Color { color: CDDATerFurnColor },
  Bgcolor { bgcolor: CDDATerFurnColor },
}

impl Default for CDDATerFurnColorSelect {
  fn default() -> CDDATerFurnColorSelect {
    CDDATerFurnColorSelect::Color {
      color: CDDATerFurnColor::Single("".to_string()),
    }
  }
}

impl CDDATerFurnColorSelect {
  pub fn is_default(t: &CDDATerFurnColorSelect) -> bool {
    t == &CDDATerFurnColorSelect::default()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnWorkbench {
  /**
   * @docs JSON_INFO    speed when manufacturing
   * @srcs mapdata.cpp  furn_workbench_info   default 1.0
   */
  pub multiplier: f64,
  /**
   * @docs JSON_INFO      allowed mass       exceeding allowed mass/volume will give panalty
   * @srcs mapdata.cpp    furn_workbench_info    default max
   */
  pub mass: CDDAMass,
  /**
   * @docs JSON_INFO     allowed volume
   * @srcs mapdata.cpp    furn_workbench_info     default max
   */
  pub volume: CDDAVolume,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnPlant {
  /**
   * @docs JSON_INFO     transforms to after planted
   * @srcs mapdata.cpp    plant_info   default null
   */
  pub transform: String,
  /**
   * @docs JSON_INFO     transforms to after harvested
   * @srcs mapdata.cpp    plant_info   default null
   */
  pub base: String,
  /**
   * @docs JSON_INFO     modify growth speed
   * @srcs mapdata.cpp    plant_info   default 1.0
   */
  #[serde(default = "float64::default_1")]
  #[serde(skip_serializing_if = "float64::is_default_1")]
  pub growth_multiplier: f64,
  /**
   * @docs JSON_INFO     modify harvest amount
   * @srcs mapdata.cpp    plant_info   default 1.0
   */
  #[serde(default = "float64::default_1")]
  #[serde(skip_serializing_if = "float64::is_default_1")]
  pub harvest_multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnFieldBashCommon {
  /**
   * @srcs mapdata.cpp    map_bash_info   default -1  min str required to bash
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub str_min: i64,
  /**
   * @srcs mapdata.cpp    map_bash_info   default -1  max str required:
   *                 bash succeeds if str >= random # between str_min & str_max
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub str_max: i64,
  /**
   * @srcs mapdata.cpp    map_bash_info   default -1   alternate values for has_adjacent_furniture(...) == true
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub str_min_blocked: i64,
  /**
   * @srcs mapdata.cpp    map_bash_info   default -1   alternate values for has_adjacent_furniture(...) == true
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub str_max_blocked: i64,
  /**
   * @srcs mapdata.cpp    map_bash_info   default -1  Alternative values for floor supported by something from below
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub str_min_supported: i64,
  /**
   * @srcs mapdata.cpp    map_bash_info   default -1  Alternative values for floor supported by something from below
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub str_max_supported: i64,
  /**
   * @docs JSON_INFO  If greater than 0, destroying the object causes an explosion with this strength
   * @srcs mapdata.cpp    map_bash_info   default 0
   */
  #[serde(default = "int64::default_0")]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub explosive: i64,
  /**
   * @docs JSON_INFO  sound volume when bashed
   * @srcs mapdata.cpp    map_bash_info   default -1
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub sound_vol: i64,
  /**
   * @docs JSON_INFO  sound volume when failing bashing
   * @srcs mapdata.cpp    map_bash_info   default -1
   */
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub sound_fail_vol: i64,
  /**
   * @docs JSON_INFO    Radius of the tent supported by this tile
   * @srcs mapdata.cpp    map_bash_info   default 1
   */
  #[serde(default = "int64::default_1")]
  #[serde(skip_serializing_if = "int64::is_default_1")]
  pub collapse_radius: i64,
  /**
   * @srcs mapdata.cpp    map_bash_info   default false   Only used for destroying, not normally bashable
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_false")]
  pub destroy_only: bool,
  /**
   * @srcs mapdata.cpp    map_bash_info   default false   This terrain is the roof of the tile below it, try to destroy that too
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_false")]
  pub bash_below: bool,
  /**
   * @docs JSON_INFO    sound message
   * @srcs mapdata.cpp    map_bash_info   default "smash!"
   */
  #[serde(default)]
  pub sound: String,
  /**
   * @docs JSON_INFO    sound message when failing
   * @srcs mapdata.cpp    map_bash_info   default "thump!"
   */
  #[serde(default)]
  pub sound_fail: String,
  /**
   * @docs JSON_INFO    For furniture that is part of tents, this defines the id of the center part
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub tent_centers: Vec<String>,
  /**
   * @docs JSON_INFO    An item group (inline) or an id of an item group   default   "EMPTY_GROUP"
   */
  #[serde(default)]
  pub items: CDDABashDeconItems,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnBash {
  #[serde(flatten)]
  pub bash_common: CDDATerFurnFieldBashCommon,
  /**
   * @docs JSON_INFO    The furniture that will be set after the original has been deconstructed. default "f_null"
   */
  #[serde(default)]
  pub furn_set: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerFurnDeconCommon {
  /**
   * @srcs mapdata.cpp    map_deconstruct_info   default false This terrain provided a roof, we need to tear it down now
   */
  #[serde(default)]
  pub deconstruct_above: bool,

  // /**
  // * @srcs mapdata.cpp    map_bash_info   always true for now
  // */
  // #[serde(default)]
  // pub can_do: bool,
  /**
   * @docs JSON_INFO    An item group (inline) or an id of an item group   default   "EMPTY_GROUP"
   */
  #[serde(default)]
  pub items: CDDABashDeconItems,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnDecon {
  #[serde(flatten)]
  pub deconstruct_common: CDDATerFurnDeconCommon,
  /**
   * @docs JSON_INFO    The furniture that will be set after the original has been deconstructed. default "f_null"
   */
  #[serde(default)]
  pub furn_set: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDABashDeconItems {
  Id(String),
  Collection(Vec<CDDABashDeconItem>),
}

impl Default for CDDABashDeconItems {
  fn default() -> CDDABashDeconItems {
    CDDABashDeconItems::Id("EMPTY_GROUP".to_string())
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDABashDeconItem {
  pub item: String,
  #[serde(default = "CDDAIntRange::default_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_1")]
  pub count: CDDAIntRange,
  #[serde(default = "CDDAIntRange::default_0")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_0")]
  pub charges: CDDAIntRange,
  #[serde(default = "int64::default_100")]
  #[serde(skip_serializing_if = "int64::is_default_100")]
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
pub struct CDDATerFurnShoot {
  /**
   * @srcs mapdata.cpp  map_shoot_info   damage reduction range to apply to shot when hit
   */
  pub reduce_damage: (i64, i64),
  /**
   * @srcs mapdata.cpp  map_shoot_info   damage reduction range to apply to laser shot when hit
   */
  pub reduce_damage_laser: (i64, i64),
  /**
   * @srcs mapdata.cpp  map_shoot_info   damage range required to have a chance to destroy
   */
  pub destroy_damage: (i64, i64),
  /**
   * @srcs mapdata.cpp  map_shoot_info   Base chance to hit the object at all (defaults to 100%)
   */
  #[serde(default = "int64::default_100")]
  #[serde(skip_serializing_if = "int64::is_default_100")]
  pub chance_to_hit: i64,
  /**
   * @srcs mapdata.cpp  map_shoot_info   Are lasers incapable of destroying the object (defaults to false)
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_false")]
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
   * @srcs mapdata.cpp    activity_data_common     time used
   */
  #[serde(default = "CDDATime::default_1s")]
  #[serde(skip_serializing_if = "CDDATime::is_default_1s")]
  pub duration: CDDATime,
  /**
   * @srcs mapdata.cpp    activity_data_common    message
   */
  #[serde(default)]
  pub message: String,
  /**
   * @srcs mapdata.cpp    activity_data_common    sound
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
