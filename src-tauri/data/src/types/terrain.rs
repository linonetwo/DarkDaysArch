use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};
use crate::common::*;
use super::furniture::*;

pub type CDDATerrainArray = Vec<CDDATerrain>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrain {
  #[serde(flatten)]
  pub ter_furn_common: CDDATerFurnCommon,
  //terrain unique key
  #[serde(flatten)]
  pub ter_omittable: CDDATerrainOmittable,

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
#[serde(untagged)]
pub enum CDDATerrainOmittable {
  Optional(CDDATerrainOptional),
  Mandatory(CDDATerrainMandatory),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainMandatory {
  pub name: CDDAName,

  pub description: String,

  pub move_cost: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainOptional {
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

  #[serde(default)]
  pub move_cost: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainBash {
  #[serde(flatten)]
  pub bash_common: CDDATerFurnFieldBashCommon,

  #[serde(default)]
  pub ter_set: String,

  #[serde(default)]
  pub ter_set_bashed_from_above: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainDecon {
  #[serde(flatten)]
  pub deconstruct_common: CDDATerFurnDeconCommon,

  #[serde(default)]
  pub ter_set: String,
}
