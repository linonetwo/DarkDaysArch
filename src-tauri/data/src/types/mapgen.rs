use super::palette::*;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use crate::common::{CDDAIntRange, int64};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenWithCache {
  /**
   * Full mapgen file content, for code editor to display
   */
  pub raw_mapgen: CDDAMapgenArray,
  /**
   * Map 2D array that have place-holder characters replaced with actual item ID, for map view to display
   * And we have multiple mapgen in a file, so this will be a 3D matrix.
   * But each location can have terrain, furniture, item and so on, so each tile will be a list, so this is a 4D tensor
   */
  pub parsed_map: Vec<Vec<Vec<Vec<ItemIDOrItemList>>>>,
}

/**
 * A char in map rows can mean multiple item, like # mean a terrain and a furniture, and some terrain can have id same as a furniture, so we have to keep id's type in a tuple
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ItemIDOrItemList {
  /**
   * (type, id), where type is like "terrain" or "furniture"
   */
  Id((MapgenPaletteKeys, String)),
  ItemList(Vec<(MapgenPaletteKeys, String)>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum MapgenPaletteKeys {
  #[allow(non_camel_case_types)]
  terrain,
  #[allow(non_camel_case_types)]
  furniture,
  #[allow(non_camel_case_types)]
  items,
}

pub type CDDAMapgenArray = Vec<CDDAMapgen>;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgen {
  Om(CDDAMapgenOM),
  Update(CDDAMapgenUpdate),
  Nested(CDDAMapgenNested),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenCommon {
  #[serde(rename = "type")]
  pub type_field: String,
  #[serde(default)]
  pub method: String,
  pub object: Option<CDDAMapgenObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenOM {
  pub om_terrain: String,
  #[serde(flatten)]
  pub common: CDDAMapgenCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenUpdate {
  pub update_mapgen_id: String,
  #[serde(flatten)]
  pub common: CDDAMapgenCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenNested {
  pub nested_mapgen_id: String,
  #[serde(flatten)]
  pub common: CDDAMapgenCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenObject {
  pub fill_ter: String,
  pub rows: Vec<String>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub palettes: Vec<String>,
  #[serde(flatten)]
  pub mapping_object: CDDAMapgenMapping,

  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_monsters: Vec<CDDAMapgenPlaceMonsters>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_item: Vec<CDDAMapgenPlaceItem>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_items: Vec<CDDAMapgenPlaceItems>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_fields: Vec<CDDAMapgenPlaceFields>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_npcs: Vec<CDDAMapgenPlaceNpcs>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_signs: Vec<CDDAMapgenPlaceSigns>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_vendingmachines: Vec<CDDAMapgenPlaceVendings>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_toilets: Vec<CDDAMapgenPlaceToilets>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_liquids: Vec<CDDAMapgenPlaceLiquids>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_gaspumps: Vec<CDDAMapgenPlaceGaspumps>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_vehicles: Vec<CDDAMapgenPlaceVehicles>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_traps: Vec<CDDAMapgenPlaceTraps>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_furniture: Vec<CDDAMapgenPlaceFurniture>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_terrain: Vec<CDDAMapgenPlaceTerrain>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_monster: Vec<CDDAMapgenPlaceMonster>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_rubble: Vec<CDDAMapgenPlaceRubble>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_computers: Vec<CDDAMapgenPlaceComputers>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_nested: Vec<CDDAMapgenPlaceNested>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_graffiti: Vec<CDDAMapgenPlaceGraffiti>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub translate_ter: Vec<CDDAMapgenTranslateTer>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_zones: Vec<CDDAMapgenPlaceZones>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_ter_furn_tranforms: Vec<CDDAMapgenPlaceTransforms>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub faction_owner: Vec<CDDAMapgenFactionOwner>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub place_loot: Vec<CDDAMapgenPlaceLoot>,
  #[serde(default)]
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub set: Vec<CDDAMapgenSet>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenCoor {
  pub x: CDDAIntRange,
  pub y: CDDAIntRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenCoor2 {
  pub x2: CDDAIntRange,
  pub y2: CDDAIntRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceMonsters {
  #[serde(flatten)]
  pub arguments: CDDAPaletteMonstersValueMonster,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceItems {
  #[serde(flatten)]
  pub arguments: CDDAPaletteItemsValueItem,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceItem {
  #[serde(flatten)]
  pub arguments: CDDAPaletteItemValueItem,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceFields {
  #[serde(flatten)]
  pub arguments: CDDAPaletteFieldsValueField,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceNpcs {
  #[serde(flatten)]
  pub arguments: CDDAPaletteNpcsValueNpc,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceSigns {
  #[serde(flatten)]
  pub arguments: CDDAPaletteSignsValueSign,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceVendings {
  #[serde(flatten)]
  pub arguments: CDDAPaletteVendingsValueVending,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceToilets {
  #[serde(flatten)]
  pub arguments: CDDAPaletteToiletsValueToilet,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceLiquids {
  #[serde(flatten)]
  pub arguments: CDDAPaletteLiquidsValueLiquid,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceGaspumps {
  #[serde(flatten)]
  pub arguments: CDDAPaletteGaspumpsValueGaspump,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceVehicles {
  #[serde(flatten)]
  pub arguments: CDDAPaletteVehiclesValueVehicle,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceMonster {
  #[serde(flatten)]
  pub arguments: CDDAPaletteMonsterValueMonster,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceRubble {
  #[serde(flatten)]
  pub arguments: CDDAPaletteRubbleValueRubble,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceComputers {
  #[serde(flatten)]
  pub arguments: CDDAPaletteComputersValueComputer,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceNested {
  #[serde(flatten)]
  pub arguments: CDDAPaletteNestedValueNested,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceGraffiti {
  #[serde(flatten)]
  pub arguments: CDDAPaletteGraffitiValueGraffiti,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceZones {
  #[serde(flatten)]
  pub arguments: CDDAPaletteZonesValueZone,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceTransforms {
  #[serde(flatten)]
  pub arguments: CDDAPaletteTransformsValueTransform,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceTerrain {
  #[serde(flatten)]
  pub arguments: CDDAPaletteTerrainValueTerrain,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceFurniture {
  #[serde(flatten)]
  pub arguments: CDDAPaletteFurnitureValueFurniture,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceTraps {
  #[serde(flatten)]
  pub arguments: CDDAPaletteTrapsValueTrap,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenTranslateTer {
  #[serde(flatten)]
  pub arguments: CDDAPaletteTranslateValueTranslate,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenFactionOwner {
  #[serde(flatten)]
  pub arguments: CDDAPaletteFactionValueFaction,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenPlaceLoot {
  Item(CDDAMapgenPlaceLootItem),
  Group(CDDAMapgenPlaceLootGroup)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceLootCommon {
  /**
   * @srcs mapgen.cpp   jmapgen_loot    "chance": x means    x% int or min-max   default 100
   */
  #[serde(default = "CDDAIntRange::default_100")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_100")]
  pub chance: CDDAIntRange,
  /**
   * @srcs  mapgen.cpp jmapgen_loot  int or min-max  default 1
   */
  #[serde(default = "CDDAIntRange::default_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_1")]
  pub repeat: CDDAIntRange,

  #[serde(default = "int64::default_0")]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub ammo: i64,

  #[serde(default = "int64::default_0")]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub magazine: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceLootItem {
  pub item: String,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
  #[serde(flatten)]
  pub loot_common: CDDAMapgenPlaceLootCommon,
  /**
   * @srcs  mapgen.cpp jmapgen_loot  used for gun variant
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "String::is_empty")]
  pub variant: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenPlaceLootGroup {
  pub group: String,
  #[serde(flatten)]
  pub coordinate: CDDAMapgenCoor,
  #[serde(flatten)]
  pub loot_common: CDDAMapgenPlaceLootCommon,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAMapgenSetCommon {
  /**
   * @srcs  mapgen.cpp    mapgen_function_json_base::setup_setmap
   */
  #[serde(default = "CDDAIntRange::default_1")]
  #[serde(skip_serializing_if = "CDDAIntRange::is_default_1")]
  pub repeat: CDDAIntRange,
  #[serde(default = "int64::default_1")]
  #[serde(skip_serializing_if = "int64::is_default_1")]
  pub chance: i64,
  #[serde(default = "int64::default_0")]
  #[serde(skip_serializing_if = "int64::is_default_0")]
  pub rotation: i64,
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub fuel: i64,
  #[serde(default = "int64::default_m1")]
  #[serde(skip_serializing_if = "int64::is_default_m1")]
  pub status: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum CDDAMapgenSet {
  Point(CDDAMapgenSetPoint),
  Line(CDDAMapgenSetLine),
  Square(CDDAMapgenSetSquare),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "point")]
#[allow(non_camel_case_types)]
pub enum CDDAMapgenSetPoint {
  terrain{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  furniture{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  trap{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  radiation{
    amount: CDDAIntRange,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  bash{
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "line")]
#[allow(non_camel_case_types)]
pub enum CDDAMapgenSetLine {
  terrain{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  furniture{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  trap{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  radiation{
    amount: CDDAIntRange,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  bash{
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "square")]
#[allow(non_camel_case_types)]
pub enum CDDAMapgenSetSquare {
  terrain{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  furniture{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  trap{
    id: String,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  radiation{
    amount: CDDAIntRange,
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
  bash{
    #[serde(flatten)]
    coordinate: CDDAMapgenCoor,
    #[serde(flatten)]
    coordinate2: CDDAMapgenCoor2,
    #[serde(flatten)]
    set_common: CDDAMapgenSetCommon,
  },
}