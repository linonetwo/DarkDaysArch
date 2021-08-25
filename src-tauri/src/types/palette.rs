use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;
use schemars::JsonSchema;

pub type CDDAPaletteArray = Vec<CDDAPalette>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAPalette {
  #[serde(rename = "type")]
  pub type_field: String,
  pub id: String,

  #[serde(rename = "//")]
  #[serde(default)]
  pub comment: String,

  #[serde(default)]
  pub parameters: BTreeMap<String, CDDAPaletteParametersValue>,

  // we can only use mapping in palette
  #[serde(flatten)]
  pub mapping_object: CDDAMapgenMapping,
  
  /**
   * @docs MAPGEN.md   everything using mapping can be included
   */
  #[serde(default)]
  pub mapping: BTreeMap<String,CDDAMapgenMapping>,

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAMapgenMapping {
  #[serde(default)]
  pub terrain: BTreeMap<String, CDDAPaletteTerrainValue>,
  /**
   * @example "furniture": { "c": "f_exercise", "u": [ "f_ergometer", "f_ergometer_mechanical" ]}
   */
  #[serde(default)]
  pub furniture: BTreeMap<String, CDDAPaletteFurnitureValue>,
  #[serde(default)]
  pub items: BTreeMap<String, CDDAPaletteItemsValue>,
  #[serde(default)]
  pub monsters: BTreeMap<String, Vec<CDDAPaletteMonstersValue>>,
  #[serde(default)]
  pub fields: BTreeMap<String, Vec<CDDAPaletteFieldsValue>>,
  #[serde(default)]
  pub npcs: BTreeMap<String, Vec<CDDAPaletteNpcsValue>>,
  #[serde(default)]
  pub signs: BTreeMap<String, Vec<CDDAPaletteSignsValue>>,
  #[serde(default)]
  pub vendingmachines: BTreeMap<String, Vec<CDDAPaletteVendingsValue>>,
  #[serde(default)]
  pub liquids: BTreeMap<String, CDDAPaletteLiquidsValue>,
  #[serde(default)]
  pub vehicles: BTreeMap<String, CDDAPaletteVehiclesValue>,
  #[serde(default)]
  pub item: BTreeMap<String, CDDAPaletteItemValue>,
  #[serde(default)]
  pub monster: BTreeMap<String, Vec<CDDAPaletteMonsterValue>>,
  #[serde(default)]
  pub traps: BTreeMap<String, Vec<CDDAPaletteTrapsValue>>,
  #[serde(default)]
  pub rubble: BTreeMap<String, Vec<CDDAPaletteRubbleValue>>,
  /**
   * @example "toilets": { "&": { "amount": [ 0, 40 ] } }
   */
  #[serde(default)]
  pub toilets: BTreeMap<String, CDDAPaletteToiletsValue>,
  #[serde(default)]
  pub gaspumps: BTreeMap<String, CDDAPaletteGaspumpsValue>,
  #[serde(rename = "sealed_item")]
  #[serde(default)]
  pub sealed_item: BTreeMap<String, CDDAPaletteSealedValue>,
  #[serde(default)]
  pub graffiti: BTreeMap<String, CDDAPaletteGraffitiValue>,
  #[serde(default)]
  pub zones: BTreeMap<String, CDDAPaletteZonesValue>,
  // #[serde(rename = "translate_ter")]
  // used only in mapgen
  // #[serde(default)]
  // pub translate_ter: BTreeMap<String, CDDAPaletteTranslateValue>,
  #[serde(rename = "ter_furn_transforms")]
  #[serde(default)]
  pub ter_furn_transforms: BTreeMap<String, CDDAPaletteTransformValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CDDAPaletteParametersValue {
  #[serde(rename = "type")]
  pub type_field: String,
  pub default: CDDAPaletteParametersValueDefault,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteDistribution {
  Id(String),
  IdList(Vec<String>),
  IdWithWeight(String, i64),
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
pub struct CDDAPaletteParameterReference {
  pub param: String,
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
  FieldList(Vec<CDDAPaletteFieldsValueField>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteSignsValue {
  Sign(CDDAPaletteSignsValueSign),
  SignList(Vec<CDDAPaletteSignsValueSign>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteVendingsValue {
  Vending(CDDAPaletteVendingsValueVending),
  VendingList(Vec<CDDAPaletteVendingsValueVending>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteToiletsValue {
  Toilet(CDDAPaletteToiletsValueToilet),
  ToiletList(Vec<CDDAPaletteToiletsValueToilet>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteGaspumpsValue {
  Gaspump(CDDAPaletteGaspumpsValueGaspump),
  GaspumpList(Vec<CDDAPaletteGaspumpsValueGaspump>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteVehiclesValue {
  Vehicle(CDDAPaletteVehiclesValueVehicle),
  VehicleList(Vec<CDDAPaletteVehiclesValueVehicle>)
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAPaletteMonsterValue {
  Monster(CDDAPaletteMonsterValueMonster),
  MonsterList(Vec<CDDAPaletteMonsterValueMonster>),
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
  SignList(Vec<CDDAPaletteGraffitiValueGraffiti>)
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
pub enum CDDAPaletteTransformValue {
  Transform(CDDAPaletteTransformValueTransform),
  TransformList(Vec<CDDAPaletteTransformValueTransform>),
}

// only used in update_mapgen
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTransformValueTransform {
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
  #[serde(rename = "rubble_type")]
  #[serde(default)]
  pub rubble_type: String,
  /**
   * @docs MAPGEN.md    bashing items will or not   default is false
   */
  #[serde(default)]
  pub items: bool,
  /**
   * @docs MAPGEN.md    ter id     default is t_dirt
   */
  #[serde(rename = "floor_type")]
  #[serde(default)]
  pub floor_type: String,
  /**
   * @docs MAPGEN.md    if true it just writes on top of what currently exists  default false
   */
  #[serde(default)]
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
  pub name: String,
}

// at least one key    not used!!!
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteGraffitiValueGraffiti {
  /**
   * @docs MAPGEN.md    the massage shown
   */
  #[serde(default)]
  pub signage: String,
  /**
   * @docs MAPGEN.md   a category of snippet?
   */
  #[serde(default)]
  pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteSealedValueSealedItem {
  pub furniture: String,
  /**
   * @docs MAPGEN.md    single item
   */
  #[serde(default)]
  pub item: Option<CDDAPaletteItemValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteSealedValueSealedItems {
  pub furniture: String,
  /**
   * @docs MAPGEN.md    item group
   */
  #[serde(default)]
  pub items: Option<CDDAPaletteItemsValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTrapsValueTrap {
  pub trap: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteTerrainValueTerrain {
  pub terrain: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteFurnitureValueFurniture {
  pub furniture: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonsterValueMonster {
  pub monster: String,
  /**
   * @docs MAPGEN.md    default is false
   */
  #[serde(default)]
  pub friendly: bool,
  /**
   * @docs MAPGEN.md    special name default is nameless 
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default  "None"?
   */
  #[serde(default)]
  pub name: String,
  /**
   * @docs MAPGEN.md    a mission target or not   
   * @srcs mapgen.cpp     jmapgen_monster   constructor  default false
   */
  #[serde(default)]
  pub target: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteItemValueItem {
  pub item: String,
  /**
   * @docs MAPGEN.md    "chance": x means   one in x  int or min-max   default 1
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub chance: CDDAIntRangeOne,
  /**
   * @docs MAPGEN.md   int or min-max  default 1
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub amount: CDDAIntRangeOne,
  /**
   * @docs MAPGEN.md   int or min-max  default 1
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub repeat: CDDAIntRangeOne,

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
  pub feul:String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteGaspumpsValueGaspump {
  /**
   * @docs MAPGEN.md    default is random? or zero?
   */
  #[serde(default)]
  pub amount: CDDAIntRangeOne,
  /**
   * @docs MAPGEN.md    type of fuel   default unknown   gasoline maybe?
   */
  #[serde(default)]
  pub feul:String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteToiletsValueToilet {
  /**
   * @docs MAPGEN.md    default is random? or zero?
   */
  #[serde(default)]
  pub amount: CDDAIntRangeOne,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteVendingsValueVending {
  /**
   * @docs MAPGEN.md    default randomly choose one of "vending_food" or "vending_drink"
   */
  #[serde(rename = "item_group")]
  #[serde(default)]
  pub item_group: String,
  /**
   * @docs MAPGEN.md    "chance": x means    x% int or min-max   default 1    1% low prob
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub chance: CDDAIntRangeOne,
  /**
   * @docs MAPGEN.md   360 degree 
   */
  #[serde(default)]
  pub rotation: i64,
  /**
   * @docs MAPGEN.md    fuel status    default -1   -1 means 1%-7% fuel
   */
  #[serde(default)]
  pub feul: i64,
  /**
   * @docs MAPGEN.md    body status    default -1   -1 means light damage    0 perfect    1 heavy damage
   */
  #[serde(default)]
  pub status: i64,
}

// at least one key
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteSignsValueSign {
  /**
   * @docs MAPGEN.md    the massage shown
   */
  #[serde(default)]
  pub signage: String,
  /**
   * @docs MAPGEN.md   a category of snippet?
   */
  #[serde(default)]
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
  pub target: bool,
  /**
   * @docs MAPGEN.md   default 0
   */
  #[serde(default)]
  pub add_trait: CDDAStringArray,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteFieldsValueField {
  pub field: String,
  /**
   * @docs MAPGEN.md    default 1, possible 1, 2, 3 or more        originally density in old version
   */
  #[serde(default)]
  pub intensity: i64,
  /**
   * @docs MAPGEN.md   default 0
   */
  #[serde(default)]
  pub age: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteItemsValueItem {
  pub item: String,
  /**
   * @docs MAPGEN.md    "chance": x means    x% int or min-max   default 100
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeHun::is_default")]
  pub chance: CDDAIntRangeHun,
  /**
   * @docs MAPGEN.md   int or min-max  default 1
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub repeat: CDDAIntRangeOne,

  #[serde(default)]
  pub ammo: i64,
  #[serde(default)]
  pub magazine: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteLiquidsValue {
  pub liquid: String,
  pub amount: (i64, i64),
  /**
   * @docs MAPGEN.md    "chance": x means    one in x  int or min-max
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub chance: CDDAIntRangeOne,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAPaletteMonstersValueMonster {
  pub monster: String,
  /**
   * @docs MAPGEN.md    "chance": x means    one in x   int or min-max
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "CDDAIntRangeOne::is_default")]
  pub chance: CDDAIntRangeOne,
  /**
   * @docs MAPGEN.md    if not present, use default density limited by distance between city center
   * use -1.0 to represent default
   */
  #[serde(default = "default_m1_f64")]
  // #[serde(skip_serializing_if = "is_default_m1_f64")]
  // #[serde(default)]
  pub density: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAStringArray {
  // without unit
  Single(String),
  // with unit
  Multiple(Vec<String>),
}
impl Default for CDDAStringArray {
  fn default() -> Self {
    CDDAStringArray::Single(String::new())
  }
}

// integer or range default with 1
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAIntRangeOne {
  // without unit
  Single(i64),
  // with unit
  Range((i64, i64)),
}
impl CDDAIntRangeOne {
  pub fn is_default(&self) -> bool {
    self == &CDDAIntRangeOne::Single(1)
  }
}
impl Default for CDDAIntRangeOne {
  fn default() -> Self {
    CDDAIntRangeOne::Single(1)
  }
}

// integer or range default with 100
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAIntRangeHun {
  // without unit
  Single(i64),
  // with unit
  Range((i64, i64)),
}
impl CDDAIntRangeHun {
  pub fn is_default(&self) -> bool {
    self == &CDDAIntRangeHun::Single(100)
  }
}
impl Default for CDDAIntRangeHun {
  fn default() -> Self {
    CDDAIntRangeHun::Single(100)
  }
}

fn default_m1_f64() -> f64 {
  -1.0
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
  t == &T::default()
}

fn is_default_m1_f64(t: &f64) -> bool {
  t == &-0.1
}