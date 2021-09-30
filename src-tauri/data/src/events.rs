pub mod mod_loading;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/**
 * Type root for json schema export, in this way we can export all types together
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum EventRootObject {
  ModLoadingEventPayload(mod_loading::ModLoadingEventPayload),
}
