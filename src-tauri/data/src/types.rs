use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod city_building;
pub mod external_option;
pub mod furniture;
pub mod mapgen;
pub mod mod_info;
pub mod overmap_connection;
pub mod overmap_location;
pub mod overmap_special;
pub mod overmap_terrain;
pub mod palette;
pub mod region_settings;
pub mod terrain;
pub mod tileset;
pub mod trap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
#[serde(untagged)]
pub enum CDDA_JSON {
  Furniture(furniture::CDDAFurniture),
  Mapgen(mapgen::CDDAMapgen),
  OvermapTerrain(overmap_terrain::CDDAOvermapTerrain),
  OvermapSpecial(overmap_special::CDDAOvermapSpecial),
  OvermapConnection(overmap_connection::CDDAOvermapConnection),
  OvermapLocation(overmap_location::CDDAOvermapLocation),
  CityBuilding(city_building::CDDACityBuilding),
  RegionSettings(region_settings::CDDARegionSettings),
  Palette(palette::CDDAPalette),
  Terrain(terrain::CDDATerrain),
  Tileset(tileset::CDDATileSetConfig),
}

#[allow(non_camel_case_types)]
pub type CDDA_JSON_Array = Vec<CDDA_JSON>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAKnowledgeGraph {
  pub furniture: furniture::CDDAFurniture,
  pub mapgen: mapgen::CDDAMapgen,
  pub overmap_terrain: overmap_terrain::CDDAOvermapTerrain,
  pub overmap_special: overmap_special::CDDAOvermapSpecial,
  pub overmap_connection: overmap_connection::CDDAOvermapConnection,
  pub overmap_location: overmap_location::CDDAOvermapLocation,
  pub city_building: city_building::CDDACityBuilding,
  pub region_settings: region_settings::CDDARegionSettings,
  pub palette: palette::CDDAPalette,
  pub terrain: terrain::CDDATerrain,
  pub tileset: tileset::CDDATileSetConfig,
}
