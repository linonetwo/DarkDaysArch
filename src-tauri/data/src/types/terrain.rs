use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};
use crate::common::*;
use super::furniture::*;

pub type CDDATerrainArray = Vec<CDDATerrain>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATerrain {
  //type using color
  Color(CDDATerrainCr),
  //type using bgcolor
  Background(CDDATerrainBg),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainBase {
  #[serde(flatten)]
  pub ter_furn_common: CDDATerFurnCommon,
  //terrain unique key

  /**
   * @docs JSON_INFO.md   actual move cost is carreied by multiplying it by 50
   */
  pub move_cost: i64,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub bash: Option<CDDATerrainBash>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub deconstruct: Option<CDDATerrainDecon>,
  
  /**
   * @docs JSON_INFO.md   0 means no heat, 1 is equal to fire of intensity 1
   */
  #[serde(default)]
  pub heat_radiation: i64,

  /**
   * @docs JSON_INFO.md   
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub lockpick_result: String,

  /**
   * @docs JSON_INFO.md   
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub lockpick_message: String,

  /**
   * @docs JSON_INFO.md    build-in trap
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub trap: String,

  /**
   * @docs JSON_INFO.md    used for havest
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub transforms_into: String,
  /**
   * @docs JSON_INFO.md    build-in trap
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub harvest_by_season: Vec<CDDATerFurnHarvest>,

  /**
   * @docs JSON_INFO.md    default roof
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub roof: String,

  /**
   * @docs mapdata.cpp    ter_t    ???
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub allowed_template_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainCr {
  pub color: CDDATerFurnColor,

  #[serde(flatten)]
  pub base: CDDATerrainBase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainBg {
  pub bgcolor: CDDATerFurnColor,

  #[serde(flatten)]
  pub base: CDDATerrainBase,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainBash {
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
  pub destroy_only: bool,

  #[serde(default)]
  pub bash_below: bool,

  #[serde(default)]
  pub sound: String,

  #[serde(default)]
  pub sound_fail: String,

  #[serde(default)]
  pub ter_set: String,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainDecon {
  #[serde(default)]
  pub deconstruct_above: bool,

  #[serde(default)]
  pub can_do: bool,

  #[serde(default)]
  pub ter_set: String,

  #[serde(default)]
  pub items: Vec<CDDABashDeconItem>,
}
