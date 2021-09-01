use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum CDDAJSONType {
  #[allow(non_camel_case_types)]
  MOD_INFO_Literal,
  #[allow(non_camel_case_types)]
  furniture_Literal,
  #[allow(non_camel_case_types)]
  mapgen_Literal,
  #[allow(non_camel_case_types)]
  overmap_special_Literal,
  #[allow(non_camel_case_types)]
  overmap_terrain_Literal,
  #[allow(non_camel_case_types)]
  palette_Literal,
  #[allow(non_camel_case_types)]
  terrain_Literal,
  #[allow(non_camel_case_types)]
  trap_Literal,
  #[allow(non_camel_case_types)]
  region_settings_Literal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum furniture_Literal {
  #[allow(non_camel_case_types)]
  furniture,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum mapgen_Literal {
  #[allow(non_camel_case_types)]
  mapgen,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum overmap_special_Literal {
  #[allow(non_camel_case_types)]
  overmap_special,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum overmap_terrain_Literal {
  #[allow(non_camel_case_types)]
  overmap_terrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum palette_Literal {
  #[allow(non_camel_case_types)]
  palette,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum terrain_Literal {
  #[allow(non_camel_case_types)]
  terrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum MOD_INFO_Literal {
  #[allow(non_camel_case_types)]
  MOD_INFO,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum trap_Literal {
  #[allow(non_camel_case_types)]
  trap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum region_settings_Literal {
  #[allow(non_camel_case_types)]
  region_settings,
}