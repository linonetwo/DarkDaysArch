/**
 * Events about sending message from rust to JS
 */
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

/**
 * game://mod-loading
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModLoadingEventPayload {
  /** This is the type of this event, won't have actuarial data, just sync this string between rs and ts */
  #[serde(rename = "game://mod-loading")]
  type_field: Option<String>,
  fileList: Vec<ModLoadingEventFileItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ModLoadingEventFileItem {
  mod_name: String,
  file_count: i32,
  progress: i32,
}
