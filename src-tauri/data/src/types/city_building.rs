use super::overmap_special::CDDAOvermapSpecialOvermap;
use crate::common::string::city_building_Literal;
use crate::common::*;
use crate::list::SelectListItem;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

pub type CDDACityBuildingArray = Vec<CDDACityBuilding>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDACityBuilding {
  #[serde(rename = "type")]
  pub cdda_json_type: city_building_Literal,

  #[serde(flatten)]
  pub select_list: SelectListItem,
  /**
   * @docs OVERMAP  As in `overmap_special`, with one caveat: all point x and y values must be >= 0.
   */
  pub overmaps: Vec<CDDAOvermapSpecialOvermap>,
  /**
   * @docs OVERMAP  List of `overmap_location` ids that this overmap terrain may be placed on.
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
   * @docs OVERMAP and FLAG for overmap special    enum needed
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub flags: Vec<String>,
}

impl CDDACityBuilding {
  pub fn get_id(&self) -> Option<Vec<String>> {
    let select_list = &self.select_list;
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
      None => { return None; }
    };
    Some(result)
  }
}