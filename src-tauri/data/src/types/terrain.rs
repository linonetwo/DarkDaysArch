use super::furniture::*;
use crate::common::string::terrain_Literal;
use crate::common::*;
use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};

pub type CDDATerrainArray = Vec<CDDATerrain>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrain {
  #[serde(rename = "type")]
  pub cdda_json_type: terrain_Literal,

  #[serde(flatten)]
  pub ter_furn_common: CDDATerFurnCommon,
  /**
   * @docs  JSON_INFO.md  Move cost to move through. A value of 0 means it's impassable (e.g. wall).
   * You should not use negative values. The positive value is multiple of 50 move points
   */
  #[serde(default)]
  pub move_cost: i64,
  /**
   * @docs JSON_INFO.md   if not defined, cannot be bashed to broken
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bash: Option<CDDATerrainBash>,
  /**
   * @docs JSON_INFO.md   if not defined, cannot be deconstructed
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deconstruct: Option<CDDATerrainDecon>,
  /**
   * @docs JSON_INFO.md   Heat emitted for a terrain. A value of 0 means no fire. A value of 1 equals a fire of intensity of 1.
   * @srcs mapdata.cpp    map_data_common_t     default 0
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub heat_radiation: i64,

  /**
   * @docs JSON_INFO.md    When the terrain is successfully lockpicked, this is the terrain it will turn into.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub lockpick_result: String,

  /**
   * @docs JSON_INFO.md   When the terrain is successfully lockpicked, this is the message that will be printed to the player.
   * When it is missing, a generic `"The lock opens…"` message will be printed instead.
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
   * @docs JSON_INFO.md    havest thing by season
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub harvest_by_season: Vec<CDDATerrainHarvest>,

  /**
   * @docs JSON_INFO.md    default roof
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub roof: String,

  /**
   * @srcs mapdata.cpp    ter_t    ???
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub allowed_template_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainBash {
  #[serde(flatten)]
  pub bash_common: CDDATerFurnFieldBashCommon,
  /**
   * @docs JSON_INFO    ter_set" is only used upon "deconstruct" entries in terrain and is mandatory there.
   */
  pub ter_set: String,
  /**
   * @srcs mapdata.cpp    terrain to set if bashed from above (defaults to ter_set)
   */
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ter_set_bashed_from_above: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainDecon {
  #[serde(flatten)]
  pub deconstruct_common: CDDATerFurnDeconCommon,
  /**
   * @docs JSON_INFO    ter_set" is only used upon "deconstruct" entries in terrain and is mandatory there.
   */
  #[serde(default)]
  pub ter_set: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATerrainHarvest {
  /**
   * @docs JSON_INFO.md    in this seasons, item group with id can be got
   */
  //TODO: enum ?
  pub seasons: Vec<String>,
  /**
   * @docs JSON_INFO.md    item group
   */
  pub id: String,
}

impl CDDATerrain {
  pub fn get_id(&self) -> Option<Vec<String>> {
    let select_list = &self.ter_furn_common.select_list;
    let mut result:Vec<String> = Vec::new();
    match &select_list.id {
      Some(id_mix) => {
        match id_mix {
          CDDAStringArray::Single(id) => {
            result.push((*id).clone());
          },
          CDDAStringArray::Multiple(ids) => {
            for id in ids {
              result.push((*id).clone());
            }
          }
        }
      },
      None => {
        match &select_list.abstract_id {
          Some(ab_id) => {
            result.push((*ab_id).clone());
          },
          None => { return None; }
        }
      }
    };
    Some(result)
  }
}