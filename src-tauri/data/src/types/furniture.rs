use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};

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
pub struct CDDAFurnitureBase {
  //mandatory
  #[serde(rename = "type")]
  pub type_field: String,

  pub id: String,

  pub name: CDDAName,

  pub description: String,

  pub symbol: String,

  pub move_cost_mod: i64,

  pub required_str: i64,

  //optional

  //furniture unique key
  #[serde(default)]
  pub light_emitted: i64,

  #[serde(default)]
  pub coverage: i64,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub crafting_pseudo_item: String,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub workbench: Option<CDDAFurnWorkbench>,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub plant_data: Option<CDDAFurnPlant>,

  #[serde(default)]
  pub surgery_skill_multiplier: f64,

  //furn ter common key
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub looks_like: String,

  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub examine_action: String,

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bash: Option<CDDAFurnBash>,

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

  // #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_volume: Option<CDDAVolume>,

  #[serde(default)]
  pub flags: Vec<String>,

  #[serde(default)]
  pub connects_to: Vec<String>,

  #[serde(default)]
  pub comfort: i64,

  #[serde(default)]
  pub floor_bedding_warmth: i64,

  #[serde(default)]
  pub bonus_fire_warmth_feet: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnitureCr {
  pub color: CDDAColor,

  #[serde(flatten)]
  pub base: CDDAFurnitureBase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAFurnitureBg {
  pub bgcolor: CDDAColor,

  #[serde(flatten)]
  pub base: CDDAFurnitureBase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATranslation {
  //emmmmm
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
pub struct CDDAFurnBash {
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
  pub furn_set: String,

  #[serde(default)]
  pub tent_centers: Vec<String>,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
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
pub struct CDDAFurnDecon {
  #[serde(default)]
  pub deconstruct_above: bool,

  #[serde(default)]
  pub can_do: bool,

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
