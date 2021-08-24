use serde;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use typescript_type_def::{TypeDef};

pub type CDDAPaletteArray = Vec<CDDAPalette>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAPalette {
  #[serde(rename = "type")]
  pub type_field: String,
  pub id: String,
  pub terrain: BTreeMap<String, CDDAPaletteTerrainValue>,
  #[serde(rename = "//")]
  #[serde(default)]
  pub comment: String,
  #[serde(default)]
  pub parameters: BTreeMap<String, CDDAPaletteParametersValue>,
  /**
   * @example "toilets": { "&": { "amount": [ 0, 40 ] } }
   */
  #[serde(default)]
  pub toilets: BTreeMap<String, BTreeMap<String, Vec<i64>>>,
  #[serde(default)]
  pub furniture: BTreeMap<String, CDDAPaletteFurnitureValue>,
  #[serde(default)]
  pub liquids: BTreeMap<String, CDDAPaletteLiquidsValue>,
  #[serde(default)]
  pub items: BTreeMap<String, CDDAPaletteItemValue>,
  #[serde(default)]
  pub monsters: BTreeMap<String, Vec<CDDAPaletteMonstersValue>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(rename_all = "camelCase")]
pub struct CDDAPaletteParametersValue {
  #[serde(rename = "type")]
  pub type_field: String,
  pub default: CDDAPaletteParametersValueDefault,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAPaletteDistribution {
  Id(String),
  IdList(Vec<String>),
  IdWithWeight(String, i64),
  RecursiveMixed(Vec<CDDAPaletteDistributionMixed>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAPaletteDistributionMixed {
  Id(String),
  IdWithWeight(String, i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAPaletteFurnitureValue {
  Id(String),
  RandomList(CDDAPaletteDistribution),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAPaletteTerrainValue {
  Id(String),
  RandomList(CDDAPaletteDistribution),
  ParamRef(CDDAPaletteTerrainValueParameterReference),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
pub struct CDDAPaletteTerrainValueParameterReference {
  pub param: String,
  pub fallback: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
pub struct CDDAPaletteParametersValueDefault {
  pub distribution: CDDAPaletteDistribution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
#[serde(untagged)]
pub enum CDDAPaletteItemValue {
  Item(CDDAPaletteItemsValueItem),
  ItemList(Vec<CDDAPaletteItemsValueItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
pub struct CDDAPaletteItemsValueItem {
  item: String,
  chance: i64,
  #[serde(default)]
  repeat: (i64, i64),
  #[serde(default)]
  ammo: i64,
  #[serde(default)]
  magazine: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
pub struct CDDAPaletteLiquidsValue {
  liquid: String,
  amount: (i64, i64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TypeDef)]
pub struct CDDAPaletteMonstersValue {
  monster: String,
  chance: i64,
}
