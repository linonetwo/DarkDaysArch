use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum CDDAJSONType {
  MOD_INFO_Literal,
  furniture_Literal,
  mapgen_Literal,
  overmap_special_Literal,
  overmap_terrain_Literal,
  overmap_location_Literal,
  overmap_connection_Literal,
  palette_Literal,
  terrain_Literal,
  trap_Literal,
  EXTERNAL_OPTION_Literal,
  ITEM_BLACKLIST_Literal,
  region_settings_Literal,
  city_building_Literal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum furniture_Literal {
  furniture,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum mapgen_Literal {
  mapgen,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum overmap_special_Literal {
  overmap_special,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum overmap_terrain_Literal {
  overmap_terrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum overmap_connection_Literal {
  #[allow(non_camel_case_types)]
  overmap_connection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum overmap_location_Literal {
  #[allow(non_camel_case_types)]
  overmap_location,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum palette_Literal {
  palette,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum terrain_Literal {
  terrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum MOD_INFO_Literal {
  MOD_INFO,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum trap_Literal {
  trap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum EXTERNAL_OPTION_Literal {
  EXTERNAL_OPTION,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum ITEM_BLACKLIST_Literal {
  ITEM_BLACKLIST,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum region_settings_Literal {
  region_settings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum city_building_Literal {
  #[allow(non_camel_case_types)]
  city_building,
}
