pub mod bool;
pub mod float64;
pub mod int64;
pub mod string;

use serde::{self, Deserialize, Serialize};
// use std::collections::BTreeMap;
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAIntRange {
  // without unit
  Single(i64),
  // with unit
  Range(Vec<i64>),
}
impl CDDAIntRange {
  pub fn default_int_range_1() -> CDDAIntRange {
    CDDAIntRange::Single(1)
  }

  pub fn is_default_int_range_1(t: &CDDAIntRange) -> bool {
    t == &CDDAIntRange::default_int_range_1()
  }

  pub fn default_int_range_100() -> CDDAIntRange {
    CDDAIntRange::Single(100)
  }

  pub fn is_default_int_range_100(t: &CDDAIntRange) -> bool {
    t == &CDDAIntRange::default_int_range_100()
  }

  pub fn default_int_range_0() -> CDDAIntRange {
    CDDAIntRange::Single(0)
  }

  pub fn is_default_int_range_0(t: &CDDAIntRange) -> bool {
    t == &CDDAIntRange::default_int_range_0()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAStringArray {
  // without unit
  Single(String),
  // with unit
  Multiple(Vec<String>),
}

impl CDDAStringArray {
  pub fn default_string_array() -> CDDAStringArray {
    CDDAStringArray::Multiple(Vec::<String>::new())
  }

  pub fn is_default_string_array(t: &CDDAStringArray) -> bool {
    t == &CDDAStringArray::default_string_array()
  }
}

// fn is_default<T: Default + PartialEq>(t: &T) -> bool {
//   t == &T::default()
// }


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAName {
  Name(String),
  Translation(CDDATranslation),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMass {
  // without unit
  Wounit(i64),
  // with unit
  Wunit(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAVolume {
  // without unit
  Wounit(i64),
  // with unit
  Wunit(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATime {
  // without unit
  Wounit(i64),
  // with unit
  Wunit(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATranslation {
  // Name(String),
  pub aaa: String,
}