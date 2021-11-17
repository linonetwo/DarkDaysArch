use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{params, Connection, Result};
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

macro_rules! get_cluster {
  ($m:expr, $cluster:ident, $p:expr, $($x:path),*) => {
    let type_field = $m.get_type();
    match $m {
      $(
        $x(item) => {
          match item.get_id() {
            Some(ids) => {
              for id in ids {
                $cluster.push(CDDADataCluster{
                  id: id.clone(),
                  type_field: type_field.clone(),
                  path: $p.clone(),
                  data: (*$m).clone(),
                });
                };
              },
            None => {}
          }
        },
      )*
      _ => {},
    };
  }
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

//serialize to sqlite
impl ToSql for CDDA_JSON {
  fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
    Ok(ToSqlOutput::from(serde_json::to_string(self).unwrap()))
  }
}

impl FromSql for CDDA_JSON {
  fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
    serde_json::from_str(value.as_str().unwrap()).map_err(|err| FromSqlError::Other(Box::new(err)))
  }
}

//deserialize from sqlite
impl CDDA_JSON {
  pub fn create_cluster(&self, path: String) -> Vec<CDDADataCluster> {
    let mut cluster: Vec<CDDADataCluster> = vec![];
    get_cluster!(
      self,
      cluster,
      path,
      CDDA_JSON::furniture,
      CDDA_JSON::overmap_terrain,
      CDDA_JSON::overmap_special,
      CDDA_JSON::overmap_connection,
      CDDA_JSON::overmap_location,
      CDDA_JSON::city_building,
      CDDA_JSON::region_settings,
      CDDA_JSON::palette,
      CDDA_JSON::terrain
    );
    cluster
  }

  // get the type of CDDA_JSON
  pub fn get_type(&self) -> String {
    match self {
      CDDA_JSON::furniture(_) => {
        return "furniture".to_string();
      }
      CDDA_JSON::mapgen(_) => {
        return "mapgen".to_string();
      }
      CDDA_JSON::overmap_terrain(_) => {
        return "overmap_terrain".to_string();
      }
      CDDA_JSON::overmap_special(_) => {
        return "overmap_special".to_string();
      }
      CDDA_JSON::overmap_location(_) => {
        return "overmap_location".to_string();
      }
      CDDA_JSON::overmap_connection(_) => {
        return "overmap_connection".to_string();
      }
      CDDA_JSON::city_building(_) => {
        return "city_building".to_string();
      }
      CDDA_JSON::region_settings(_) => {
        return "region_settings".to_string();
      }
      CDDA_JSON::palette(_) => {
        return "palette".to_string();
      }
      CDDA_JSON::terrain(_) => {
        return "terrain".to_string();
      }
      CDDA_JSON::Unknown => {
        return "unknown".to_string();
      }
    }
  }
}

#[allow(non_camel_case_types)]
pub type CDDA_JSON_Array = Vec<CDDA_JSON>;

//all information needed put in the database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDADataCluster {
  pub id: String,
  pub type_field: String,
  pub path: String,
  pub data: CDDA_JSON,
}

#[derive(Debug)]
pub struct CDDAKnowledgeGraph {
  pub database: Connection,
}

impl CDDAKnowledgeGraph {
  pub fn new() -> CDDAKnowledgeGraph {
    let new_instance = CDDAKnowledgeGraph {
      database: Connection::open_in_memory().unwrap(),
    };
    new_instance.create_table();
    new_instance
  }

  // create the table before first insert
  pub fn create_table(&self) {
    self
      .database
      .execute(
        "CREATE TABLE IF NOT EXISTS 'jsonTable'(
      'Id'     TEXT NOT NULL,
      'Type'   TEXT NOT NULL,
      'Path'   TEXT NOT NULL,
      'Data'   TEXT NOT NULL,
      PRIMARY KEY ('Id','Type','Path')
    )",
        params![],
      )
      .unwrap();
    self
      .database
      .execute(
        "CREATE TABLE IF NOT EXISTS 'modFolder' (
      'Path'   TEXT NOT NULL,
      PRIMARY KEY ('Path')
    )",
        params![],
      )
      .unwrap();
  }

  pub fn set_mod_folder_loaded(&self, path: String) {
    self
      .database
      .execute(
        "INSERT OR REPLACE INTO 'modFolder' (
          'Path'
        ) VALUES (
          ?1
        )",
        params![path],
      )
      .unwrap();
  }

  pub fn check_mod_folder_loaded(&self, path: &String) -> bool {
    match self.database.execute("SELECT * FROM 'modFolder' WHERE 'Path' = ?1", params![path]) {
      Ok(_) => {
        true
      }
      Err(_) => {
        false
      }
    }
  }

  // insert a row
  pub fn insert(&self, data: &CDDADataCluster) {
    let sql: &str = "INSERT INTO 'jsonTable'('Id','Type','Path','Data') VALUES (?1,?2,?3,?4)";

    self.database.execute(sql, params![data.id, data.type_field, data.path, data.data]).unwrap();
  }

  //delete a row
  pub fn delete(&self, id: String, type_field: String, path: String) {
    let sql: &str = "DELETE FROM 'jsonTable' WHERE Id = ?1 AND Type = ?2 AND Path = ?3";

    self.database.execute(sql, params![id, type_field, path]).unwrap();
  }

  //update the data of a row except id type and path
  pub fn update(&self, id: String, type_field: String, path: String, data: &CDDA_JSON) {
    let sql: &str = "UPDATE 'jsonTable' SET Data = ?1 WHERE Id = ?2 AND Type = ?3 AND Path = ?4";

    self.database.execute(sql, params![data, id, type_field, path]).unwrap();
  }

  //search rows in table, only for id type path, all optional
  pub fn search(&self, id: Option<String>, type_field: Option<String>, path: Option<String>) -> Vec<CDDADataCluster> {
    let mut data_cluster_list: Vec<CDDADataCluster> = Vec::new();

    let sql: &str = &format!(
      "SELECT * FROM 'jsonTable' WHERE Id LIKE '%{}%' AND Type LIKE '%{}%' AND Path LIKE '%{}%'",
      id.unwrap_or_default(),
      type_field.unwrap_or_default(),
      path.unwrap_or_default()
    )[..];

    let mut stmt = self.database.prepare(sql).unwrap();

    let iterator = stmt
      .query_map([], |row| {
        Ok(CDDADataCluster {
          id: row.get(0).unwrap(),
          type_field: row.get(1).unwrap(),
          path: row.get(2).unwrap(),
          data: row.get(3).unwrap(),
        })
      })
      .unwrap();

    for item in iterator {
      data_cluster_list.push(item.unwrap());
    }

    data_cluster_list
  }

  // display the content of the table
  pub fn display(&self) {
    let sql: &str = "SELECT * FROM 'jsonTable'";
    let mut stmt = self.database.prepare(sql).unwrap();
    let iter = stmt
      .query_map(params![], |row| {
        Ok(CDDADataCluster {
          id: row.get(0).unwrap(),
          type_field: row.get(1).unwrap(),
          path: row.get(2).unwrap(),
          data: row.get(3).unwrap(),
        })
      })
      .unwrap();

    for i in iter {
      println!("{:?}", i.unwrap());
    }
  }
}
