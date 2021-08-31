use crate::common::CDDAName;
use schemars::JsonSchema;
use serde;
use serde::{Deserialize, Serialize};

pub type SelectListArray = Vec<SelectList>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
// #[serde(allow_unknown)]
pub struct SelectList {
  #[serde(rename = "type")]
  pub type_field: String,
  pub id: String,
  #[serde(default)]
  pub name: CDDAName,
}
