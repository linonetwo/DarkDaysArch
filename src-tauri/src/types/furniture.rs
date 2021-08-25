use serde;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub type CDDAFurnArray = Vec<CDDAFurniture>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAFurniture {
  //type using color
  Color(CDDAFurnitureCr),
  //type using bgcolor
  Background(CDDAFurnitureBg),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnitureBase {
  //mandatory
  #[serde(rename = "type")]
  pub type_field: String,

  pub id: String,

  pub name: CDDAName,

  pub description: String,

  pub symbol: String,

  #[serde(rename = "move_cost_mod")]
  pub move_cost_mod: i64,

  #[serde(rename = "required_str")]
  pub required_str: i64,

  //optional

  //furniture unique key
  #[serde(rename = "light_emitted")]
  #[serde(default)]
  pub light_emitted: i64,

  #[serde(default)]
  pub coverage: i64,

  #[serde(rename = "crafting_pseudo_item")]
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub crafting_pseudo_item: String,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub workbench: Option<CDDAFurnWorkbench>,

  #[serde(rename = "plant_data")]
  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plant_data: Option<CDDAFurnPlant>,

  #[serde(rename = "surgery_skill_multiplier")]
  #[serde(default)]
  pub surgery_skill_multiplier: f64,

  //furn ter common key
  #[serde(rename = "looks_like")]
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub looks_like: String,

  #[serde(rename = "examine_action")]
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub examine_action: String,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bash: Option<CDDAFurnBash>,
  #[serde(rename = "map_bash_info")]
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub map_bash_info: String,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deconstruct: Option<CDDAFurnDecon>,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub close: String,
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub open: String,

  #[serde(rename = "max_volume")]
  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_volume: Option<CDDAVolume>,

  #[serde(default)]
  pub flags: Vec<String>,

  #[serde(rename = "connects_to")]
  #[serde(default)]
  pub connects_to: Vec<String>,

  #[serde(default)]
  pub comfort: i64,
  #[serde(rename = "floor_bedding_warmth")]
  #[serde(default)]
  pub floor_bedding_warmth: i64,
  #[serde(rename = "bonus_fire_warmth_feet")]
  #[serde(default)]
  pub bonus_fire_warmth_feet: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnitureCr {
  pub color: CDDAColor,

  #[serde(flatten)]
  pub base: CDDAFurnitureBase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnitureBg {
  pub bgcolor: CDDAColor,

  #[serde(flatten)]
  pub base: CDDAFurnitureBase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDATranslation {
  //emmmmm
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnWorkbench {
  pub multiplier: f64,
  pub mass: CDDAMass,
  pub volume: CDDAVolume,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnPlant {
  pub transform: String,
  pub base: String,
  #[serde(rename = "growth_multiplier")]
  pub growth_multiplier: f64,
  #[serde(rename = "harvest_multiplier")]
  pub harvest_multiplier: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnBash {
  #[serde(rename = "str_min")]
  #[serde(default)]
  pub str_min: i64,
  #[serde(rename = "str_max")]
  #[serde(default)]
  pub str_max: i64,

  #[serde(rename = "str_min_blocked")]
  #[serde(default)]
  pub str_min_blocked: i64,
  #[serde(rename = "str_max_blocked")]
  #[serde(default)]
  pub str_max_blocked: i64,

  #[serde(rename = "str_min_supported")]
  #[serde(default)]
  pub str_min_supported: i64,
  #[serde(rename = "str_max_supported")]
  #[serde(default)]
  pub str_max_supported: i64,

  #[serde(default)]
  pub explosive: i64,

  #[serde(rename = "sound_vol")]
  #[serde(default)]
  pub sound_vol: i64,

  #[serde(rename = "sound_fail_vol")]
  #[serde(default)]
  pub sound_fail_vol: i64,

  #[serde(rename = "collapse_radius")]
  #[serde(default)]
  pub collapse_radius: i64,

  #[serde(rename = "destroy_only")]
  #[serde(default)]
  pub destroy_only: bool,

  #[serde(rename = "bash_below")]
  #[serde(default)]
  pub bash_below: bool,

  #[serde(default)]
  pub sound: String,
  #[serde(rename = "sound_fail")]
  #[serde(default)]
  pub sound_fail: String,

  #[serde(rename = "furn_set")]
  #[serde(default)]
  pub furn_set: String,

  #[serde(rename = "tent_centers")]
  #[serde(default)]
  pub tent_centers: Vec<String>,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDABashDeconItem {
  pub item: String,
  pub count: Option<CDDAIntRange>,
  pub charges: Option<CDDAIntRange>,
  #[serde(default)]
  pub prob: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAFurnDecon {
  #[serde(rename = "deconstruct_above")]
  #[serde(default)]
  pub deconstruct_above: bool,

  #[serde(rename = "can_do")]
  #[serde(default)]
  pub can_do: bool,

  #[serde(rename = "furn_set")]
  #[serde(default)]
  pub furn_set: String,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAName {
  Name(String),
  Translation(CDDATranslation),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMass {
  // without unit
  Wounit(i64),
  // with unit
  Wunit(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAVolume {
  // without unit
  Wounit(i64),
  // with unit
  Wunit(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAColor {
  // one color
  Single(String),
  // four colors for four season, 2, 3 not sure
  Multiple(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAIntRange {
  // without unit
  Single(i64),
  // with unit
  Range((i64, i64)),
}
