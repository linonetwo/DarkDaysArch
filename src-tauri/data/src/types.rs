use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod furniture;
pub mod mapgen;
pub mod overmap_special;
pub mod overmap_terrain;
pub mod palette;
pub mod region_settings;
pub mod terrain;
pub mod tileset;
pub mod trap;
pub mod mod_info;
pub mod external_option;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CDDA_JSON {
  Furniture(furniture::CDDAFurniture),
  Mapgen(mapgen::CDDAMapgen),
  OvermapTerrain(overmap_terrain::CDDAOvermapTerrain),
  Palette(palette::CDDAPalette),
  Terrain(terrain::CDDATerrain),
  Tileset(tileset::CDDATileSetConfig),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAKnowledgeGraph {
  pub furniture: furniture::CDDAFurniture,
  pub mapgen: mapgen::CDDAMapgen,
  pub overmap_terrain: overmap_terrain::CDDAOvermapTerrain,
  pub palette: palette::CDDAPalette,
  pub terrain: terrain::CDDATerrain,
  pub tileset: tileset::CDDATileSetConfig,
}
