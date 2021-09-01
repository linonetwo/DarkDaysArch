pub mod bool;
pub mod float64;
pub mod int64;
pub mod string;
pub mod tuple;

use serde::{self, Deserialize, Serialize};
// use std::collections::BTreeMap;
use schemars::JsonSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAIntRange {
  // single int
  Single(i64),
  // with size 1 or 2
  Range(Vec<i64>),
}
impl CDDAIntRange {
  pub fn default_1() -> CDDAIntRange {
    CDDAIntRange::Single(1)
  }

  pub fn is_default_1(t: &CDDAIntRange) -> bool {
    t == &CDDAIntRange::default_1()
  }

  pub fn default_100() -> CDDAIntRange {
    CDDAIntRange::Single(100)
  }

  pub fn is_default_100(t: &CDDAIntRange) -> bool {
    t == &CDDAIntRange::default_100()
  }

  pub fn default_0() -> CDDAIntRange {
    CDDAIntRange::Single(0)
  }

  pub fn is_default_0(t: &CDDAIntRange) -> bool {
    t == &CDDAIntRange::default_0()
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
  pub fn default() -> CDDAStringArray {
    CDDAStringArray::Multiple(Vec::<String>::new())
  }

  pub fn is_default(t: &CDDAStringArray) -> bool {
    t == &CDDAStringArray::default()
  }
}

// pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
//   t == &T::default()
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAName {
  Name(String),
  Translation(CDDATranslation),
}

// used only for copy-from
impl Default for CDDAName {
  fn default() -> CDDAName {
    CDDAName::Name("".to_string())
  }
}

impl CDDAName {
  pub fn is_default(t: &CDDAName) -> bool {
    t == &CDDAName::default()
  }
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

impl CDDAVolume {
  pub fn default_1000l() -> CDDAVolume {
    CDDAVolume::Wounit(1000000)
  }

  pub fn is_default_1000l(t: &CDDAVolume) -> bool {
    t == &CDDAVolume::default_1000l()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDATime {
  // without unit
  Wounit(i64),
  // with unit
  Wunit(String),
}

impl CDDATime {
  pub fn default_1s() -> CDDATime {
    CDDATime::Wounit(1)
  }

  pub fn is_default_1s(t: &CDDATime) -> bool {
    t == &CDDATime::default_1s()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDATranslation {
  #[serde(default)]
  #[serde(rename = "str")]
  pub string: String,
  #[serde(default)]
  #[serde(rename = "//NOLINT(cata-text-style)")]
  pub cata_style: String,
}
// used to change some value whose type supporting copy-from
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDACopyChange {
  #[serde(default)]
  pub flags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Interval<T>(T,T);

impl<T: Default+PartialEq> Default for Interval<T> {
  fn default() -> Interval<T> {
    Interval::<T>(T::default(),T::default())
  }
}

impl<T:Default+PartialEq> Interval<T> {
  pub fn is_default(t: &Interval<T>) -> bool {
    t == &Interval::<T>::default()
  }

  pub fn default_int_0m1() -> Interval<i64> {
    Interval::<i64>(0,-1)
  }

  pub fn is_default_int_0m1(t: &Interval<i64>) -> bool {
    t == &Interval::<i64>::default_int_0m1()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TriPoint<T>(T,T,T);

impl<T: Default+PartialEq> Default for TriPoint<T> {
  fn default() -> TriPoint<T> {
    TriPoint::<T>(T::default(),T::default(),T::default())
  }
}

impl<T: Default+PartialEq> TriPoint<T> {
  pub fn is_default(t: &TriPoint<T>) -> bool {
    t == &TriPoint::<T>::default()
  }
}