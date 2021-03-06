use crate::common::string::{EXTERNAL_OPTION_Literal, ITEM_BLACKLIST_Literal};
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use serde_json::value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAExternalOption {
  #[serde(rename = "type")]
  pub cdda_json_type: EXTERNAL_OPTION_Literal,
  pub name: String,
  pub info: Option<String>,
  pub stype: String,
  pub value: value::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAItemBlackList {
  #[serde(rename = "type")]
  pub cdda_json_type: ITEM_BLACKLIST_Literal,
  pub whitelist: bool,
  pub items: Vec<Option<serde_json::Value>>,
}
