use serde;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type CDDAPaletteArray = Vec<CDDAPalette>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAPalette {
  #[serde(rename = "type")]
  pub type_field: String,
  pub id: String,
  pub terrain: BTreeMap<String, CDDAPaletteTerrainValue>,
  #[serde(rename = "//")]
  pub comment: Option<String>,
  pub parameters: Option<BTreeMap<String, CDDAPaletteParametersValue>>,
  /**
   * @example "toilets": { "&": { "amount": [ 0, 40 ] } }
   */
  pub toilets: Option<BTreeMap<String, BTreeMap<String, Vec<i64>>>>,
  pub furniture: Option<BTreeMap<String, CDDAPaletteFurnitureValue>>,
  pub liquids: Option<BTreeMap<String, CDDAPaletteLiquidsValue>>,
  pub items: Option<BTreeMap<String, CDDAPaletteItemValue>>,
  pub monsters: Option<BTreeMap<String, Vec<CDDAPaletteMonstersValue>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CDDAPaletteParametersValue {
  #[serde(rename = "type")]
  pub type_field: String,
  pub default: CDDAPaletteParametersValueDefault,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CDDAPaletteDistribution {
  Id(String),
  IdList(Vec<String>),
  IdWithWeight(String, i64),
  RecursiveMixed(Vec<CDDAPaletteDistributionMixed>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CDDAPaletteDistributionMixed {
  Id(String),
  IdWithWeight(String, i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CDDAPaletteFurnitureValue {
  Id(String),
  RandomList(CDDAPaletteDistribution),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CDDAPaletteTerrainValue {
  Id(String),
  RandomList(CDDAPaletteDistribution),
  ParamRef(CDDAPaletteTerrainValueParameterReference),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAPaletteTerrainValueParameterReference {
  pub param: String,
  pub fallback: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAPaletteParametersValueDefault {
  pub distribution: CDDAPaletteDistribution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CDDAPaletteItemValue {
  Item(CDDAPaletteItemsValueItem),
  ItemList(Vec<CDDAPaletteItemsValueItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAPaletteItemsValueItem {
  item: String,
  chance: i64,
  repeat: Option<(i64, i64)>,
  ammo: Option<i64>,
  magazine: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAPaletteLiquidsValue {
  liquid: String,
  amount: (i64, i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CDDAPaletteMonstersValue {
  monster: String,
  chance: i64,
}
