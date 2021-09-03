use crate::common::string::CDDAJSONType;
use crate::common::{CDDAName, CDDAStringArray};
use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};

pub type SelectList = Vec<SelectListItemWithType>;

/**
 * Minimal information about a JSON, this is used as result when searching for a JSON.
 * We can use id in the result to request full JSON content.
 *
 * This is SelectListItem with a `type` field. SelectListItem struct is going to be `serde(flatten)` into many other struct, and the `type` field will overwrite their literal `type` field, so we need to use a `SelectListItemWithType` struct to isolate the general `type` field.
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SelectListItemWithType {
  #[serde(flatten)]
  basic_list: SelectListItem,
  #[serde(rename = "type")]
  pub cdda_json_type: CDDAJSONType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SelectListItem {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<CDDAStringArray>,
  #[serde(rename = "abstrct")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub abstract_id: Option<String>,
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAName::is_default")]
  pub name: CDDAName,
}
