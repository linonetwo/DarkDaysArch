use super::external_option::{CDDAExternalOption, CDDAItemBlackList};
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

use crate::common::string::MOD_INFO_Literal;

pub type CDDAModInfoArray = Vec<CDDAModInfoWithExternalOption>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAModInfoWithExternalOption {
  ModInfo(CDDAModInfo),
  ExternalOption(CDDAExternalOption),
  ItemBlackList(CDDAItemBlackList),
}

/**
 * @docs https://github.com/CleverRaven/Cataclysm-DDA/blob/master/doc/MODDING.md#modinfojson
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAModInfo {
  #[serde(rename = "type")]
  pub cdda_json_type: MOD_INFO_Literal,

  pub id: String,
  pub name: String,
  pub authors: Option<CDDAModInfoAuthors>,
  pub description: String,
  pub version: Option<String>,
  pub category: Option<String>,
  pub dependencies: Option<Vec<String>>,
  pub maintainers: Option<Vec<String>>,

  #[serde(rename = "//")]
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub comment: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAModInfoAuthors {
  Name(String),
  NameList(Vec<String>),
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
// pub enum CDDAModInfoCategory {
//   /** A mod that adds a lot of stuff. Typically reserved for large mods (eg: Core game files, Aftershock) */
//   #[allow(non_camel_case_types)]
//   content,
//   /** A mod that fundamentally changes the game. In particular, the assumption is that a player should not use two total conversion mods at the same time, and so they will not be tested together. However, nothing prevents players from using more than one if they wish. (eg: Dark Skies Above) */
//   #[allow(non_camel_case_types)]
//   total_conversion,
//   /** A mod that adds new items and recipes to the game (eg: More survival tools) */
//   #[allow(non_camel_case_types)]
//   items,
//   /** A mod that adds new creatures or NPCs to the game (eg: Modular turrets) */
//   #[allow(non_camel_case_types)]
//   creatures,
//   /** Miscellaneous content additions to the game (eg: Alternative map key, Crazy cataclysm) */
//   #[allow(non_camel_case_types)]
//   misc_additions,
//   #[allow(non_camel_case_types)]
//   misc,
//   #[allow(non_camel_case_types)]
//   Other,
//   /** New overmap locations and structures (eg: Fuji's more buildings). If you're blacklisting buildings from spawning, this is also a usable category (eg: No rail stations). */
//   #[allow(non_camel_case_types)]
//   buildings,
//   /** New cars or vehicle parts (eg: Tanks and other vehicles) */
//   #[allow(non_camel_case_types)]
//   vehicles,
//   /** A mod designed to rebalance the game in some way (eg: Safe autodocs). */
//   #[allow(non_camel_case_types)]
//   rebalance,
//   /** A mod that adds something magic-related to the game (eg: Necromancy) */
//   #[allow(non_camel_case_types)]
//   magical,
//   /** A mod that stops items from spawning in the world (eg: No survivor armor, No drugs) */
//   #[allow(non_camel_case_types)]
//   item_exclude,
//   /** A mod that stops certain monster varieties from spawning in the world (eg: No fungal monsters, No ants) */
//   #[allow(non_camel_case_types)]
//   monster_exclude,
//   /** A mod that adjusts game graphics in some way (eg: Graphical overmap) */
//   #[allow(non_camel_case_types)]
//   graphical,
//   #[allow(non_camel_case_types)]
//   food,
//   #[allow(non_camel_case_types)]
//   #[serde(rename = "martial arts")]
//   martial_arts,
// }
