use crate::common::{CDDAName, CDDAStringArray};
use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};

pub type SelectListArray = Vec<SelectList>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
// #[serde(allow_unknown)]
pub struct SelectList {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<CDDAStringArray>,
  #[serde(rename = "abstrct")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub abstract_id: Option<String>,
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAName::is_default")]
  pub name: CDDAName,
}
