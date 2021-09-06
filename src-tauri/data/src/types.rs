use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

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
pub enum CDDAType {
  Furniture,
  Mapgen,
  OvermapTerrain,
  OvermapSpecial,
  OvermapConnection,
  OvermapLocation,
  CityBuilding,
  RegionSettings,
  Palette,
  Terrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(non_camel_case_types)]
pub enum CDDA_JSON {
  furniture(furniture::CDDAFurniture),
  mapgen(mapgen::CDDAMapgen),
  overmap_terrain(overmap_terrain::CDDAOvermapTerrain),
  overmap_special(overmap_special::CDDAOvermapSpecial),
  overmap_connection(overmap_connection::CDDAOvermapConnection),
  overmap_location(overmap_location::CDDAOvermapLocation),
  city_building(city_building::CDDACityBuilding),
  region_settings(region_settings::CDDARegionSettings),
  palette(palette::CDDAPalette),
  terrain(terrain::CDDATerrain),
  // tileset(tileset::CDDATileSetConfig),
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
  pub file_content: BTreeMap<String,Vec<(String,CDDAType)>>,
  // pub tileset: BTreeMap<String, tileset::CDDATileSetConfig>,
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
      file_content: Default::default(),
      // tileset: Default::default(),
    }
  }

  // remind to release memory for original CDDA_JSON object
  pub fn update(&mut self, update_data: CDDA_JSON, file_path: PathBuf) {
    let file = file_path.display().to_string();
  // get the mutable inner list for a file_path, if create one if not found
    let id_clutter_list: &mut Vec<(String,CDDAType)>;
    match self.file_content.get_mut(&file) {
      Some(id_list) => {
        id_clutter_list = id_list;
      },
      None => {
        self.file_content.insert(file.clone(), Vec::new());
        id_clutter_list = self.file_content.get_mut(&file).unwrap();
      }
    }
    // create or change index
    match &update_data {
      CDDA_JSON::palette(palette_item) => {
        match palette_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.palette.insert(id.clone(),(*palette_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::Palette));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::mapgen(mapgen_item) => {
        match mapgen_item.get_id() {
          Some(id) => {
            self.mapgen.insert(id.clone(),(*mapgen_item).clone());
            id_clutter_list.push((id.clone(),CDDAType::Mapgen));
          },
          None => {}
        }
      },
      CDDA_JSON::furniture(furniture_item) => {
        match furniture_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.furniture.insert(id.clone(),(*furniture_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::Furniture));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::terrain(terrain_item) => {
        match terrain_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.terrain.insert(id.clone(),(*terrain_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::Terrain));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::overmap_terrain(omt_item) => {
        match omt_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_terrain.insert(id.clone(),(*omt_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::OvermapTerrain));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::overmap_special(oms_item) => {
        match oms_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_special.insert(id.clone(),(*oms_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::OvermapSpecial));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::overmap_location(oml_item) => {
        match oml_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_location.insert(id.clone(),(*oml_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::OvermapLocation));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::overmap_connection(omc_item) => {
        match omc_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.overmap_connection.insert(id.clone(),(*omc_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::OvermapConnection));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::city_building(city_item) => {
        match city_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.city_building.insert(id.clone(),(*city_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::CityBuilding));
            }
          },
          None => {}
        }
      },
      CDDA_JSON::region_settings(region_item) => {
        match region_item.get_id() {
          Some(ids) => {
            for id in ids {
              self.region_settings.insert(id.clone(),(*region_item).clone());
              id_clutter_list.push((id.clone(),CDDAType::RegionSettings));
            }
          },
          None => {}
        }
      },
      _ => {}
    };
  }
}