use crate::common::string::overmap_location_Literal;
use crate::common::*;
use crate::list::SelectListItem;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

pub type CDDAOvermapLocationArray = Vec<CDDAOvermapLocation>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapLocation {
  #[serde(rename = "type")]
  pub cdda_json_type: overmap_location_Literal,

  //no name allowed
  #[serde(flatten)]
  pub select_list: SelectListItem,
  /**
   * @docs OVERMAP  List of `overmap_terrain` that can be considered part of this location.
   */
  pub terrains: Vec<String>,
}

impl CDDAOvermapLocation {
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