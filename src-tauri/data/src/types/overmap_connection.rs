use crate::common::string::overmap_connection_Literal;
use crate::common::*;
use crate::list::SelectListItem;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

pub type CDDAOvermapConnectionArray = Vec<CDDAOvermapConnection>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapConnection {
  //no name allowed
  #[serde(flatten)]
  pub select_list: SelectListItem,
  /**
   * @docs OVERMAP  List of `overmap_location` ids that the special may be placed on.      
   */
  pub subtypes: Vec<CDDAOvermapConnectionSubtype>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAOvermapConnectionSubtype {
  /**
   * @docs OVERMAP  `overmap_terrain` to be placed when the placement location matches `locations`.   
   */
  pub terrain: String,
  /**
   * @docs OVERMAP  List of `overmap_location` that this subtype applies to. Can be empty; signifies `terrain` is valid as is.
   */
  pub locations: Vec<String>,
  /**
   * @docs OVERMAP  Cost of this subtype when pathfinding a route. Default 0.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub basic_cost: i64,
  /**
   * @docs OVERMAP   See `Overmap connections` in [JSON_FLAGS.md]
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub flags: Vec<String>,
}

impl CDDAOvermapConnection {
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