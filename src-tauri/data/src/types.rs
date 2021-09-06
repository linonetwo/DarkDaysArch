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
  #[serde(other)]
  Unknown,
}

#[allow(non_camel_case_types)]
pub type CDDA_JSON_Array = Vec<CDDA_JSON>;

/**
 * Get each type of JSON by id
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAKnowledgeGraph {
  pub furniture: BTreeMap<String, furniture::CDDAFurniture>,
  pub mapgen: BTreeMap<String, mapgen::CDDAMapgen>,
  pub overmap_terrain: BTreeMap<String, overmap_terrain::CDDAOvermapTerrain>,
  pub overmap_special: BTreeMap<String, overmap_special::CDDAOvermapSpecial>,
  pub overmap_connection: BTreeMap<String, overmap_connection::CDDAOvermapConnection>,
  pub overmap_location: BTreeMap<String, overmap_location::CDDAOvermapLocation>,
  pub city_building: BTreeMap<String, city_building::CDDACityBuilding>,
  pub region_settings: BTreeMap<String, region_settings::CDDARegionSettings>,
  pub palette: BTreeMap<String, palette::CDDAPalette>,
  pub terrain: BTreeMap<String, terrain::CDDATerrain>,
  pub tileset: BTreeMap<String, tileset::CDDATileSetConfig>,
}

impl CDDAKnowledgeGraph {
  pub fn new() -> CDDAKnowledgeGraph {
    CDDAKnowledgeGraph {
      furniture: Default::default(),
      mapgen: Default::default(),
      overmap_terrain: Default::default(),
      overmap_special: Default::default(),
      overmap_connection: Default::default(),
      overmap_location: Default::default(),
      city_building: Default::default(),
      region_settings: Default::default(),
      palette: Default::default(),
      terrain: Default::default(),
      tileset: Default::default(),
    }
  }

  // remind to release memory for original CDDA_JSON object
  pub fn update(&mut self, update_data: CDDA_JSON) {
    match &update_data {
      CDDA_JSON::Palette(palette_item) => {
        match palette_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.palette.insert(id,(*palette_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::Mapgen(mapgen_item) => {
        match mapgen_item.get_id() {
          Some(id) => {
            self.mapgen.insert(id,(*mapgen_item).clone());
          },
          None => {}
        }
      },
      CDDA_JSON::Furniture(furniture_item) => {
        match furniture_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.furniture.insert(id,(*furniture_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::Terrain(terrain_item) => {
        match terrain_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.terrain.insert(id,(*terrain_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::OvermapTerrain(omt_item) => {
        match omt_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_terrain.insert(id,(*omt_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::OvermapSpecial(oms_item) => {
        match oms_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_special.insert(id,(*oms_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::OvermapLocation(oml_item) => {
        match oml_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_location.insert(id,(*oml_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::OvermapConnection(omc_item) => {
        match omc_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_connection.insert(id,(*omc_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::CityBuilding(city_item) => {
        match city_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.city_building.insert(id,(*city_item).clone());
            }
          },
          None => {}
        }
      },
      CDDA_JSON::RegionSettings(region_item) => {
        match region_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.region_settings.insert(id,(*region_item).clone());
            }
          },
          None => {}
        }
      },
      _ => {}
    };
  }
}