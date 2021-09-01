use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum CDDAJSONType {
  #[allow(non_camel_case_types)]
  MOD_INFO_Literal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum MOD_INFO_Literal {
  #[allow(non_camel_case_types)]
  MOD_INFO,
}
