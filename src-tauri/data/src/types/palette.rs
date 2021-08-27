use crate::common::{bool, float64, int64, string, CDDAIntRange, CDDAStringArray};
use super::mapgen::CDDAMapgenCoor;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;

pub type CDDAPaletteArray = Vec<CDDAPalette>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPalette {
  #[serde(rename = "type")]
  pub type_field: String,
  pub id: String,

  #[serde(rename = "//")]
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub comment: String,

  // we can only use mapping in palette
  #[serde(flatten)]
  pub mapping_object: CDDAMapgenMapping,

  /**
   * @docs MAPGEN.md   everything using mapping can be included
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub mapping: BTreeMap<String, CDDAMapgenMapping>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenMapping {
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub parameters: BTreeMap<String, CDDAPaletteParametersValue>,

  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub terrain: BTreeMap<String, CDDAPaletteTerrainValue>,
  /**
   * @example "furniture": { "c": "f_exercise", "u": [ "f_ergometer", "f_ergometer_mechanical" ]}
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub furniture: BTreeMap<String, CDDAPaletteFurnitureValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub items: BTreeMap<String, CDDAPaletteItemsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub monsters: BTreeMap<String, CDDAPaletteMonstersValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub fields: BTreeMap<String, CDDAPaletteFieldsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub npcs: BTreeMap<String, CDDAPaletteNpcsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub signs: BTreeMap<String, CDDAPaletteSignsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub vendingmachines: BTreeMap<String, CDDAPaletteVendingsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub liquids: BTreeMap<String, CDDAPaletteLiquidsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub vehicles: BTreeMap<String, CDDAPaletteVehiclesValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub item: BTreeMap<String, CDDAPaletteItemValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub monster: BTreeMap<String, CDDAPaletteMonsterValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub traps: BTreeMap<String, CDDAPaletteTrapsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub rubble: BTreeMap<String, CDDAPaletteRubbleValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub computers: BTreeMap<String, CDDAPaletteComputersValue>,
  /**
   * @example "toilets": { "&": { "amount": [ 0, 40 ] } }
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub toilets: BTreeMap<String, CDDAPaletteToiletsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub gaspumps: BTreeMap<String, CDDAPaletteGaspumpsValue>,
  #[serde(rename = "sealed_item")]
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub sealed_item: BTreeMap<String, CDDAPaletteSealedValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub graffiti: BTreeMap<String, CDDAPaletteGraffitiValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub zones: BTreeMap<String, CDDAPaletteZonesValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub nested: BTreeMap<String, CDDAPaletteNestedValue>,
  // #[serde(rename = "translate_ter")]
  // used only in mapgen
  // #[serde(default)]
  // pub translate_ter: BTreeMap<String, CDDAPaletteTranslateValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub ter_furn_transforms: BTreeMap<String, CDDAPaletteTransformsValue>,
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub faction_owner_character: BTreeMap<String, CDDAPaletteFactionValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteParametersValue {
  #[serde(rename = "type")]
  pub type_field: String,
  pub default: CDDAPaletteParametersValueDefault,
}

// what happened exactly ???
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteDistribution {
  RecursiveMixed(Vec<CDDAPaletteDistributionMixed>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteDistributionMixed {
  Id(String),
  IdWithWeight(String, i64),
}
/**
 * @docs MAPGEN.md      Terrain, furniture and traps can specified as a single string, not a json object
*/

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteFurnitureValue {
  Id(String),
  Object(CDDAPaletteFurnitureValueFurniture),
  RandomList(CDDAPaletteDistribution),
  ParamRef(CDDAPaletteParameterReference),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteTerrainValue {
  Id(String),
  Object(CDDAPaletteTerrainValueTerrain),
  RandomList(CDDAPaletteDistribution),
  ParamRef(CDDAPaletteParameterReference),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteTrapsValue {
  Id(String),
  Object(CDDAPaletteTrapsValueObject),
  RandomList(CDDAPaletteDistribution),
  ParamRef(CDDAPaletteParameterReference),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteNestedValueNestedChuncks {
  Id(String),
  RandomList(CDDAPaletteDistribution),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteParameterReference {
  pub param: String,
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub fallback: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteParametersValueDefault {
  pub distribution: CDDAPaletteDistribution,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteItemsValue {
  Item(CDDAPaletteItemsValueItem),
  ItemList(Vec<CDDAPaletteItemsValueItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteNpcsValue {
  Npc(CDDAPaletteNpcsValueNpc),
  NpcList(Vec<CDDAPaletteNpcsValueNpc>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteFieldsValue {
  Field(CDDAPaletteFieldsValueField),
  FieldList(Vec<CDDAPaletteFieldsValueField>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteSignsValue {
  Sign(CDDAPaletteSignsValueSign),
  SignList(Vec<CDDAPaletteSignsValueSign>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteVendingsValue {
  Vending(CDDAPaletteVendingsValueVending),
  VendingList(Vec<CDDAPaletteVendingsValueVending>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteLiquidsValue {
  Liquid(CDDAPaletteLiquidsValueLiquid),
  LiquidList(Vec<CDDAPaletteLiquidsValueLiquid>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteToiletsValue {
  Toilet(CDDAPaletteToiletsValueToilet),
  ToiletList(Vec<CDDAPaletteToiletsValueToilet>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteGaspumpsValue {
  Gaspump(CDDAPaletteGaspumpsValueGaspump),
  GaspumpList(Vec<CDDAPaletteGaspumpsValueGaspump>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteVehiclesValue {
  Vehicle(CDDAPaletteVehiclesValueVehicle),
  VehicleList(Vec<CDDAPaletteVehiclesValueVehicle>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteItemValue {
  Item(CDDAPaletteItemValueItem),
  ItemList(Vec<CDDAPaletteItemValueItem>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteMonstersValue {
  Monster(CDDAPaletteMonstersValueMonster),
  MonsterList(Vec<CDDAPaletteMonstersValueMonster>),
}
 // confusing change ?

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteMonsterValue {
  Monster(CDDAPaletteMonsterValueMonster),
  MonsterList(Vec<CDDAPaletteMonsterValueMonster>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteMonsterValueMonster {
  Monster(CDDAPaletteMonsterValueMonsterMonster),
  Group(CDDAPaletteMonsterValueMonsterGroup),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteTrapsValueObject {
  Trap(CDDAPaletteTrapsValueTrap),
  TrapList(Vec<CDDAPaletteTrapsValueTrap>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteSealedValue {
  Sealed(CDDAPaletteSealedValueSealed),
  SealedList(Vec<CDDAPaletteSealedValueSealed>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteSealedValueSealed {
  Item(CDDAPaletteSealedValueSealedItem),
  Itemgroup(CDDAPaletteSealedValueSealedItems),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteGraffitiValue {
  Sign(CDDAPaletteGraffitiValueGraffiti),
  SignList(Vec<CDDAPaletteGraffitiValueGraffiti>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteZonesValue {
  Zone(CDDAPaletteZonesValueZone),
  ZoneList(Vec<CDDAPaletteZonesValueZone>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteRubbleValue {
  Rubble(CDDAPaletteRubbleValueRubble),
  RubbleList(Vec<CDDAPaletteRubbleValueRubble>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteTranslateValue {
  Translate(CDDAPaletteTranslateValueTranslate),
  TranslateList(Vec<CDDAPaletteTranslateValueTranslate>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteTransformsValue {
  Transform(CDDAPaletteTransformsValueTransform),
  TransformList(Vec<CDDAPaletteTransformsValueTransform>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteComputersValue {
  Computer(CDDAPaletteComputersValueComputer),
  ComputerList(Vec<CDDAPaletteComputersValueComputer>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteComputersValueComputerOptions {
  Computer(CDDAPaletteComputersValueComputerOptionsOption),
  ComputerList(Vec<CDDAPaletteComputersValueComputerOptionsOption>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteComputersValueComputerFailures {
  Computer(CDDAPaletteComputersValueComputerFailuresFailure),
  ComputerList(Vec<CDDAPaletteComputersValueComputerFailuresFailure>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteNestedValue {
  Nested(CDDAPaletteNestedValueNested),
  NestedList(Vec<CDDAPaletteNestedValueNested>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteFactionValue {
  Faction(CDDAPaletteFactionValueFaction),
  FactionList(Vec<CDDAPaletteFactionValueFaction>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteFactionValueFaction {
  /**
   * @srcs mapgen.cpp    jmapgen_nested
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteNestedValueNestedNeighbors {
  /**
   * @srcs mapgen.cpp    jmapgen_nested
   */
  #[serde(default = "CDDAStringArray::default_string_array")]
  #[serde(skip_serializing_if = "CDDAStringArray::is_default_string_array")]
  pub west: CDDAStringArray,
  #[serde(default = "CDDAStringArray::default_string_array")]
  #[serde(skip_serializing_if = "CDDAStringArray::is_default_string_array")]
  pub east: CDDAStringArray,
  #[serde(default = "CDDAStringArray::default_string_array")]
  #[serde(skip_serializing_if = "CDDAStringArray::is_default_string_array")]
  pub north: CDDAStringArray,
  #[serde(default = "CDDAStringArray::default_string_array")]
  #[serde(skip_serializing_if = "CDDAStringArray::is_default_string_array")]
  pub south: CDDAStringArray,
  #[serde(default = "CDDAStringArray::default_string_array")]
  #[serde(skip_serializing_if = "CDDAStringArray::is_default_string_array")]
  pub above: CDDAStringArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteNestedValueNested {
  /**
   * @srcs mapgen.cpp    jmapgen_nested
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub chunks: Vec<CDDAPaletteNestedValueNestedChuncks>,
  /**
   * @srcs mapgen.cpp    jmapgen_nested
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub else_chunks: Vec<CDDAPaletteNestedValueNestedChuncks>,
  /**
   * @srcs mapgen.cpp    jmapgen_nested
   */
  pub neighbors: CDDAPaletteNestedValueNestedNeighbors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteComputersValueComputerFailuresFailure {
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  pub action: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteComputersValueComputerOptionsOption {
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  pub action: String,
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  /**
   * @srcs mapgen.cpp   jmapgen_computer  default 0
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_i64_0")]
  pub security: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteComputersValueComputer {
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub access_denied: String,
  /**
   * @srcs mapgen.cpp   jmapgen_computer default 0
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "int64::is_default_i64_0")]
  pub security: i64,
  /**
   * @srcs mapgen.cpp   jmapgen_computer   default false       mission target maybe
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub target: bool,
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub options: Vec<CDDAPaletteComputersValueComputerOptions>,
  /**
   * @srcs mapgen.cpp   jmapgen_computer
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub failures: Vec<CDDAPaletteComputersValueComputerFailures>,
}

// only used in update_mapgen
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTransformsValueTransform {
  /**
   * @docs MAPGEN.md    the id of the `ter_furn_transform` to run
   */
  pub transform: String,
}

// only used in update_mapgen
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTranslateValueTranslate {
  /**
   * @docs MAPGEN.md    the terrain id of the terrain to be transformed
   */
  pub from: String,
  /**
   * @docs MAPGEN.md    the terrain id that the from terrain will transformed into
   */
  pub to: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteRubbleValueRubble {
  /**
   * @docs MAPGEN.md    furniture id     default is f_rubble
   */

  #[serde(default)]
  pub rubble_type: String,
  /**
   * @docs MAPGEN.md    bashing items will or not   default is false
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub items: bool,
  /**
   * @docs MAPGEN.md    ter id     default is t_dirt
   */

  #[serde(default)]
  pub floor_type: String,
  /**
   * @docs MAPGEN.md    if true it just writes on top of what currently exists  default false
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub overwrite: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteZonesValueZone {
  /**
   * @docs MAPGEN.md    Values: `"NPC_RETREAT"`, `"NPC_NO_INVESTIGATE"`, or `"NPC_INVESTIGATE_ONLY"`
   */
  #[serde(rename = "type")]
  pub type_field: String,
  /**
   * @docs MAPGEN.md    fraction id
   */
  pub faction: String,
  /**
   * @docs MAPGEN.md    the name of the zone
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
}

// at least one key    not used!!!
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteGraffitiValueGraffiti {
  /**
   * @docs MAPGEN.md    the massage shown
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub signage: String,
  /**
   * @docs MAPGEN.md   a category of snippet?
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteSealedValueSealedItem {
  pub furniture: String,
  /**
   * @docs MAPGEN.md    single item
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub item: Option<CDDAPaletteItemValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteSealedValueSealedItems {
  pub furniture: String,
  /**
   * @docs MAPGEN.md    item group
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub items: Option<CDDAPaletteItemsValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTrapsValueTrap {
  pub trap: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTerrainValueTerrain {
  pub terrain: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteFurnitureValueFurniture {
  pub furniture: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonsterValueMonsterSpawnDataAmmo {
  /**
   * @docs MAPGEN.md    default is false
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub ammo_id: String,
  /**
   * @docs MAPGEN.md    special name default is nameless
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default  "None"?
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub patrol: Vec<CDDAMapgenCoor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonsterValueMonsterSpawnData {
  /**
   * @docs MAPGEN.md    ammo carried
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub ammo: Vec<CDDAPaletteMonsterValueMonsterSpawnDataAmmo>,
  /**
   * @docs MAPGEN.md    patrol ??
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub patrol: Vec<CDDAMapgenCoor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonsterValueMonsterCommon {
  /**
   * @docs MAPGEN.md    default is false
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub friendly: bool,
  /**
   * @docs MAPGEN.md    special name default is nameless
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default  "None"?
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name: String,
  /**
   * @docs MAPGEN.md    a mission target or not   
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default false
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub target: bool,
  /**
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default false
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub pack_size: CDDAIntRange,
  /**
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default false   if pack_size is defined  default true ????
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub one_or_none: bool,
  /**
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default false   if pack_size is defined  default true ????
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub spawn_data: Option<CDDAPaletteMonsterValueMonsterSpawnData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonsterValueMonsterMonster {
  pub monster: CDDAPaletteDistribution,
  
  #[serde(flatten)]
  pub monster_common: CDDAPaletteMonsterValueMonsterCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonsterValueMonsterGroup {
  pub group: String,

  #[serde(flatten)]
  pub monster_common: CDDAPaletteMonsterValueMonsterCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteItemValueItem {
  pub item: String,
  /**
   * @docs MAPGEN.md    "chance": x means   one in x  int or min-max   default 1
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub chance: CDDAIntRange,
  /**
   * @docs MAPGEN.md   int or min-max  default 1
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub amount: CDDAIntRange,
  /**
   * @docs MAPGEN.md   int or min-max  default 1
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub repeat: CDDAIntRange,
  // TODO: variant and custom_flags    same for items
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteVehiclesValueVehicle {
  /**
   * @docs MAPGEN.md    if of vehicle id or vehicle group id
   */
  pub vehicle: String,
  /**
   * @docs MAPGEN.md    type of fuel   default unknown   gasoline maybe?
   */
  #[serde(default)]
  pub feul: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteGaspumpsValueGaspump {
  /**
   * @docs MAPGEN.md    default is random? or zero?
   */
  #[serde(default = "CDDAIntRange::default_int_range_0")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_0")]
  pub amount: CDDAIntRange,
  /**
   * @docs MAPGEN.md    type of fuel   default unknown   gasoline maybe?
   */
  #[serde(default)]
  pub feul: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteToiletsValueToilet {
  /**
   * @docs MAPGEN.md    default is random? or zero?
   */
  #[serde(default = "CDDAIntRange::default_int_range_0")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_0")]
  pub amount: CDDAIntRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteVendingsValueVending {
  /**
   * @docs MAPGEN.md    default randomly choose one of "vending_food" or "vending_drink"
   */

  #[serde(default)]
  pub item_group: String,
  /**
   * @docs MAPGEN.md    "chance": x means    x% int or min-max   default 1    1% low prob
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub chance: CDDAIntRange,
  /**
   * @docs MAPGEN.md   360 degree
   */
  #[serde(default = "int64::default_i64_0")]
  #[serde(skip_serializing_if = "int64::is_default_i64_0")]
  pub rotation: i64,
  /**
   * @docs MAPGEN.md    fuel status    default -1   -1 means 1%-7% fuel
   */
  #[serde(default = "int64::default_i64_m1")]
  #[serde(skip_serializing_if = "int64::is_default_i64_m1")]
  pub feul: i64,
  /**
   * @docs MAPGEN.md    body status    default -1   -1 means light damage    0 perfect    1 heavy damage
   */
  #[serde(default = "int64::default_i64_m1")]
  #[serde(skip_serializing_if = "int64::is_default_i64_m1")]
  pub status: i64,
}

// at least one key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteSignsValueSign {
  /**
   * @docs MAPGEN.md    the massage shown
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub signage: String,
  /**
   * @docs MAPGEN.md   a category of snippet?
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteNpcsValueNpc {
  // npc class id
  pub class: String,
  /**
   * @docs MAPGEN.md    a mission target or not   default false
   * @srcs mapgen.cpp     jmapgen_npc   constructor
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "bool::is_default_bool_false")]
  pub target: bool,
  /**
   * @docs MAPGEN.md   default 0
   */
  #[serde(default = "CDDAStringArray::default_string_array")]
  #[serde(skip_serializing_if = "CDDAStringArray::is_default_string_array")]
  pub add_trait: CDDAStringArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteFieldsValueField {
  pub field: String,
  /**
   * @docs MAPGEN.md    default 1, possible 1, 2, 3 or more        originally density in old version
   */
  #[serde(default = "int64::default_i64_1")]
  #[serde(skip_serializing_if = "int64::is_default_i64_1")]
  pub intensity: i64,
  /**
   * @docs MAPGEN.md   default 0
   */
  #[serde(default = "int64::default_i64_0")]
  #[serde(skip_serializing_if = "int64::is_default_i64_0")]
  pub age: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteItemsValueItem {
  pub item: String,
  /**
   * @docs MAPGEN.md    "chance": x means    x% int or min-max   default 100
   */
  #[serde(default = "CDDAIntRange::default_int_range_100")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_100")]
  pub chance: CDDAIntRange,
  /**
   * @docs MAPGEN.md   int or min-max  default 1
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub repeat: CDDAIntRange,

  #[serde(default = "int64::default_i64_0")]
  #[serde(skip_serializing_if = "int64::is_default_i64_0")]
  pub ammo: i64,

  #[serde(default = "int64::default_i64_0")]
  #[serde(skip_serializing_if = "int64::is_default_i64_0")]
  pub magazine: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteLiquidsValueLiquid {
  pub liquid: String,
  /**
   * @docs MAPGEN.md    default 0 means using certain liquid defualt amount defined
   */
  #[serde(default = "CDDAIntRange::default_int_range_0")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_0")]
  pub amount: CDDAIntRange,
  /**
   * @docs MAPGEN.md    "chance": x means    one in x  int or min-max    default 1
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub chance: CDDAIntRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonstersValueMonster {
  pub monster: String,
  /**
   * @docs MAPGEN.md    "chance": x means    one in x   int or min-max defualt 1
   */
  #[serde(default = "CDDAIntRange::default_int_range_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_int_range_1")]
  pub chance: CDDAIntRange,
  /**
   * @docs MAPGEN.md    if not present, use default density limited by distance between city center
   * use -1.0 to represent default
   */
  #[serde(default = "float64::default_f64_m1")]
  #[serde(skip_serializing_if = "float64::is_default_f64_m1")]
  pub density: f64,
}
