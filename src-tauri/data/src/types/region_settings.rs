use crate::common::*;
use crate::list::SelectListItem;
use schemars::JsonSchema;
use serde::{self, Deserialize, Serialize};
use std::collections::BTreeMap;

pub type CDDARegionSettingsArray = Vec<CDDARegionSettings>;

/**
 * @docs REGION_SETTINGS   if setting for default, all fields are required!
 */
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettings {
  #[serde(rename = "type")]
  pub cdda_json_type: string::region_settings_Literal,

  #[serde(flatten)]
  pub select_list: SelectListItem,
  /**
   * @docs REGION_SETTINGS    Default overmap terrain for this region.     overmap terrain id    vec means z-axis
   */
  pub default_oter: Vec<String>,
  /**
   * @docs REGION_SETTINGS    List of terrain types and weights applied as default ground cover.   a terrain id but a region terrain is better
   * must be a weight list
   */
  pub default_groundcover: Vec<CDDARegionSettingsDefaultGroundcover>,
  /**
   * @docs REGION_SETTINGS    Defines the resolution of regional terrain/furniture to actual types.
   */
  pub region_terrain_and_furniture: Option<CDDARegionSettingsRegionTerrainAndFurniture>,
  /**
   * @docs REGION_SETTINGS    Defines the flora that cover the `field` overmap terrain.
   */
  pub field_coverage: Option<CDDARegionSettingsFieldCoverage>,
  /**
   * @docs REGION_SETTINGS    Defines parameters for generating lakes in the region.
   */
  pub overmap_lake_settings: Option<CDDARegionSettingsOvermapLakeSettings>,
  /**
   * @docs REGION_SETTINGS    Defines parameters for generating forests and swamps in the region.
   */
  pub overmap_forest_settings: Option<CDDARegionSettingsOvermapForestSettings>,
  /**
   * @docs REGION_SETTINGS    Defines flora (and "stuff") that cover the `forest` terrain types.
   */
  pub forest_mapgen_settings: Option<CDDARegionSettingsForestMapgenSettings>,
  /**
   * @docs REGION_SETTINGS    Defines the overmap and local structure of forest trails.
   */
  pub forest_trail_settings: Option<CDDARegionSettingsForestTrailSettings>,
  /**
   * @docs REGION_SETTINGS    Defines parameters for generating lakes in the region.
   */
  pub overmap_ravine_settings: Option<CDDARegionSettingsOvermapRavineSettings>,
  /**
   * @docs REGION_SETTINGS    river scale
   */
  pub river_scale: Option<f64>,
  /**
   * @docs REGION_SETTINGS    Defines the structural compositions of cities.
   */
  pub city: Option<CDDARegionSettingsCity>,
  /**
   * @docs REGION_SETTINGS    Defines the map extra groups referenced by overmap terrains.
   */
  pub map_extras: Option<BTreeMap<String, CDDARegionSettingsMapExtras>>,
  /**
   * @docs REGION_SETTINGS    Defines the base weather attributes for the region.
   */
  pub weather: Option<CDDARegionSettingsWeather>,
  /**
   * @docs REGION_SETTINGS    Defines operations on overmap features based on their flags.
   */
  pub overmap_feature_flag_settings: Option<CDDARegionSettingsOvermapFeatureFlagSettings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsDefaultGroundcover(String, i64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsRegionTerrainAndFurniture {
  /**
   * @docs REGION_SETTINGS   List of regional terrain and their corresponding weighted lists.
   */
  pub terrain: BTreeMap<String, BTreeMap<String, i64>>,
  /**
   * @docs REGION_SETTINGS    List of regional furniture and their corresponding weighted lists.
   */
  pub furniture: BTreeMap<String, BTreeMap<String, i64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsFieldCoverage {
  /**
   * @docs REGION_SETTINGS   % of tiles in the overmap terrain that have a plant.
   */
  pub percent_coverage: f64,
  /**
   * @docs REGION_SETTINGS   Default terrain feature for plants.
   */
  pub default_ter: String,
  /**
   * @docs REGION_SETTINGS   List of features with % chance when `default_ter` isn't used.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "BTreeMap::is_empty")]
  pub other: BTreeMap<String, f64>,
  /**
   * @docs REGION_SETTINGS   % of field overmap terrains with boosted plant growth.
   */
  #[serde(default)]
  #[serde(skip_serializing_if = "float64::is_default_0")]
  pub boost_chance: f64,
  /**
   * @docs REGION_SETTINGS   % of tiles in the boosted that have a plant.   required when boost_chance is bigger than 0.0
   */
  pub boosted_percent_coverage: Option<f64>,
  /**
   * @docs REGION_SETTINGS   List of features in the boosted with % chance when `default_ter` isn't used.  required when boost_chance is bigger than 0.0
   */
  pub boosted_other: Option<BTreeMap<String, f64>>,
  /**
   * @docs REGION_SETTINGS   % of `boosted_percent_coverage` that will be covered by `boosted_other`.    required when boost_chance is bigger than 0.0
   */
  pub boosted_other_percent: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsOvermapLakeSettings {
  /**
   * @docs REGION_SETTINGS   [0, 1], x > value spawns a `lake_surface` or `lake_shore`.    default 0.25
   */
  pub noise_threshold_lake: f64,
  /**
   * @docs REGION_SETTINGS   Minimum size of the lake in overmap terrains for it to actually spawn.
   */
  pub lake_size_min: i64,
  /**
   * @docs REGION_SETTINGS   Depth of lakes, expressed in Z-levels (e.g. -1 to -10)
   */
  pub lake_depth: i64,
  /**
   * @docs REGION_SETTINGS   List of overmap terrains that can be extended to the shore if adjacent.
   */
  pub shore_extendable_overmap_terrain: Vec<String>,
  /**
   * @docs REGION_SETTINGS   Overmap terrains to treat as different overmap terrain for extending shore.
   */
  pub shore_extendable_overmap_terrain_aliases: Vec<CDDAShoreExtendableOvermapTerrainAlias>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDAShoreExtendableOvermapTerrainAlias {
  /**
   * om_terrain id    after transformation when next to a lake
   */
  pub om_terrain: String,
  /**
   * an enum maybe    match pattern
   */
  pub om_terrain_match_type: CDDAOvermapTerrainMatchType,
  /**
   * om_terrain id    before transformed when next to a lake
   */
  pub alias: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum CDDAOvermapTerrainMatchType {
  // The provided string must completely match the overmap terrain id, including
  // linear direction suffixes for linear terrain types or rotation suffixes
  // for rotated terrain types.
  #[serde(rename = "EXACT")]
  Exact,
  // The provided string must completely match the base type id of the overmap
  // terrain id, which means that suffixes for rotation and linear terrain types
  // are ignored.
  #[serde(rename = "TYPE")]
  Type,
  // The provided string must be a complete prefix (with additional parts delimited
  // by an underscore) of the overmap terrain id. For example, "forest" will match
  // "forest" or "forest_thick" but not "forestcabin".
  #[serde(rename = "PREFIX")]
  Prefix,
  // The provided string must be contained within the overmap terrain id, but may
  // occur at the beginning, end, or middle and does not have any rules about
  // underscore delimiting.
  #[serde(rename = "CONTAINS")]
  Contains,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsOvermapForestSettings {
  /**
   * @docs REGION_SETTINGS   [0, 1], x > value spawns `forest`.
   */
  pub noise_threshold_forest: f64,
  /**
   * @docs REGION_SETTINGS   [0, 1], x > value spawns `forest_thick`.
   */
  pub noise_threshold_forest_thick: f64,
  /**
   * @docs REGION_SETTINGS   [0, 1], x > value spawns `forest_water` if forest near a waterbody.
   */
  pub noise_threshold_swamp_adjacent_water: f64,
  /**
   * @docs REGION_SETTINGS   [0, 1], x > value spawns `forest_water` if forest isolated from water.
   */
  pub noise_threshold_swamp_isolated: f64,
  /**
   * @docs REGION_SETTINGS   Minimum buffer distance in overmap terrains for river floodplains.
   */
  pub river_floodplain_buffer_distance_min: i64,
  /**
   * @docs REGION_SETTINGS   Maximum buffer distance in overmap terrains for river floodplains.
   */
  pub river_floodplain_buffer_distance_max: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsForestMapgenSettings {
  /**
   * @docs REGION_SETTINGS   forest setting
   */
  pub forest: CDDARegionSettingsForestMapgenSettingsForest,
  /**
   * @docs REGION_SETTINGS   forest_thick setting
   */
  pub forest_thick: CDDARegionSettingsForestMapgenSettingsForest,
  /**
   * @docs REGION_SETTINGS   forest_water setting
   */
  pub forest_water: CDDARegionSettingsForestMapgenSettingsForest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsForestMapgenSettingsForest {
  /**
   * @docs REGION_SETTINGS   Value relative to neighbors controls how sparse the overmap terrain will be.
   */
  pub sparseness_adjacency_factor: i64,
  /**
   * @docs REGION_SETTINGS    Item group used to place items randomly within the overmap terrain.
   */
  pub item_group: String,
  /**
   * @docs REGION_SETTINGS   % chance, between 1 and 100, that an item will be placed.
   */
  pub item_group_chance: i64,
  /**
   * @docs REGION_SETTINGS   Number of times that the item spawning will be called.
   */
  pub item_spawn_iterations: i64,
  /**
   * @docs REGION_SETTINGS   clear all previously defined `groundcover` for this overmap terrain.
   */
  pub clear_groundcover: bool,
  /**
   * @docs REGION_SETTINGS   Weighted list of terrains used for base groundcover.
   */
  pub groundcover: BTreeMap<String, i64>,
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `components` for this overmap terrain.
   */
  pub clear_components: bool,
  /**
   * @docs REGION_SETTINGS   Collection of components that make up the terrains and furniture placed.
   */
  pub components: BTreeMap<String, CDDARegionSettingsForestMapgenSettingsForestComponents>,
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `terrain_furniture` for this overmap terrain.
   */
  pub clear_terrain_furniture: bool,
  /**
   * @docs REGION_SETTINGS   Collection of furniture conditionally placed based on terrain.
   */
  pub terrain_furniture: BTreeMap<String, CDDARegionSettingsForestMapgenSettingsForestTerrainFurniture>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsForestMapgenSettingsForestComponents {
  /**
   * @docs REGION_SETTINGS   Sequence in which components are processed.
   */
  pub sequence: i64,
  /**
   * @docs REGION_SETTINGS   One in X chance that something from this component will be placed.
   */
  pub chance: i64,
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `types` for this component.
   */
  pub clear_types: bool,
  /**
   * @docs REGION_SETTINGS   Weighted list of terrains and furniture that make up this component.
   */
  pub types: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsForestMapgenSettingsForestTerrainFurniture {
  /**
   * @docs REGION_SETTINGS   One in X chance that furniture from this component will be placed.
   */
  pub chance: i64,
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `furniture` for this terrain.
   */
  pub clear_furniture: bool,
  /**
   * @docs REGION_SETTINGS   Weighted list of furniture that will be placed on this terrain.
   */
  pub furniture: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsForestTrailSettings {
  /**
   * @docs REGION_SETTINGS   One in X chance a contiguous forest will have a trail system.
   */
  pub chance: i64,
  /**
   * @docs REGION_SETTINGS   One in X chance that the N/S/E/W-most point of the forest will be part of the trail system.
   */
  pub border_point_chance: i64,
  /**
   * @docs REGION_SETTINGS   Minimum contiguous forest size before a trail system can be spawned.
   */
  pub minimum_forest_size: i64,
  /**
   * @docs REGION_SETTINGS   Minimum # of random points from contiguous forest used to form trail system.
   */
  pub random_point_min: i64,
  /**
   * @docs REGION_SETTINGS   Maximum # of random points from contiguous forest used to form trail system.
   */
  pub random_point_max: i64,
  /**
   * @docs REGION_SETTINGS   Forest size is divided by this and added to the minimum number of random points.
   */
  pub random_point_size_scalar: i64,
  /**
   * @docs REGION_SETTINGS   One in X chance a trailhead will spawn at end of trail near field.
   */
  pub trailhead_chance: i64,
  /**
   * @docs REGION_SETTINGS   Maximum distance trailhead can be from a road and still be created.
   */
  pub trailhead_road_distance: i64,
  /**
   * @docs REGION_SETTINGS   Center of the trail in mapgen is offset in X and Y by a random amount between +/- variance
   */
  pub trail_center_variance: i64,
  /**
   * @docs REGION_SETTINGS   Trail width in mapgen is offset by `rng(trail_width_offset_min, trail_width_offset_max)`.
   */
  pub trail_width_offset_min: i64,
  /**
   * @docs REGION_SETTINGS   Trail width is mapgen is offset by `rng(trail_width_offset_min, trail_width_offset_max)`.
   */
  pub trail_width_offset_max: i64,
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `trail_terrain`.
   */
  pub clear_trail_terrain: bool,
  /**
   * @docs REGION_SETTINGS   Weighted list of terrain that will used for the trail.
   */
  pub trail_terrain: BTreeMap<String, i64>,
  /**
   * @docs REGION_SETTINGS   Weighted list of overmap specials / city buildings that will be placed as trailheads.
   */
  pub trailheads: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsOvermapRavineSettings {
  /**
   * @docs REGION_SETTINGS   number of varines
   */
  pub num_ravines: i64,
  /**
   * @docs REGION_SETTINGS   width of varines
   */
  pub ravine_width: i64,
  /**
   * @docs REGION_SETTINGS   range of varines
   */
  pub ravine_range: i64,
  /**
   * @docs REGION_SETTINGS   depth of varines
   */
  pub ravine_depth: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsCity {
  /**
   * @docs REGION_SETTINGS   Radial frequency of shop placement. Smaller number = more shops.
   */
  pub shop_radius: i64,
  /**
   * @docs REGION_SETTINGS   range of placement ?
   */
  pub shop_sigma: i64,
  /**
   * @docs REGION_SETTINGS   Radial frequency of park placement. Smaller number = more parks.
   */
  pub park_radius: i64,
  /**
   * @docs REGION_SETTINGS   range of placement ?
   */
  pub park_sigma: i64,
  /**
   * @docs REGION_SETTINGS   Weighted list of overmap terrains and specials used for houses.
   */
  pub houses: BTreeMap<String, i64>,
  /**
   * @docs REGION_SETTINGS   Weighted list of overmap terrains and specials used for parks.
   */
  pub parks: BTreeMap<String, i64>,
  /**
   * @docs REGION_SETTINGS   Weighted list of overmap terrains and specials used for shops.
   */
  pub shops: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsMapExtras {
  /**
   * @docs REGION_SETTINGS   One in X chance that the overmap terrain will spawn a map extra.
   */
  pub chance: i64,
  /**
   * @docs REGION_SETTINGS   Weighted list of map extras that can spawn.
   */
  pub extras: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsWeather {
  /**
   * @docs REGION_SETTINGS   Base temperature for the region in degrees Celsius.
   */
  pub base_temperature: f64,
  /**
   * @docs REGION_SETTINGS   Base humidity for the region in relative humidity %
   */
  pub base_humidity: f64,
  /**
   * @docs REGION_SETTINGS   Base pressure for the region in millibars.
   */
  pub base_pressure: f64,
  /**
   * @docs REGION_SETTINGS   Base wind for the region in mph units. Roughly the yearly average.
   */
  pub base_wind: f64,
  /**
   * @docs REGION_SETTINGS   How high the wind peaks can go. Higher values produce windier days.
   */
  pub base_wind_distrib_peaks: i64,
  /**
   * @docs REGION_SETTINGS   How the wind varies with season. Lower values produce more variation
   */
  pub base_wind_season_variation: i64,
  /**
   * @docs REGION_SETTINGS   Ids of the weather types allowed in this region.
   * When choosing weather they will be iterated over in the order they are listed and the last valid entry will be the weather.
   */
  pub weather_types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CDDARegionSettingsOvermapFeatureFlagSettings {
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `blacklist`.
   */
  pub clear_blacklist: bool,
  /**
   * @docs REGION_SETTINGS   List of flags. Any location with a matching flag will be excluded from overmap generation.
   */
  pub blacklist: Vec<String>,
  /**
   * @docs REGION_SETTINGS   Clear all previously defined `whitelist`.
   */
  pub clear_whitelist: bool,
  /**
   * @docs REGION_SETTINGS   List of flags. Only locations with a matching flag will be included in overmap generation.
   */
  pub whitelist: Vec<String>,
}


impl CDDARegionSettings {
  pub fn get_id(&self) -> Option<Vec<String>> {
    let select_list = &self.select_list;
    let mut result:Vec<String> = Vec::new();
    match &select_list.id {
      Some(id_mix) => {
        match id_mix {
          CDDAStringArray::Single(id) => {
            result.push((*id).clone());
          },
          CDDAStringArray::Multiple(ids) => {
            for id in ids {
              result.push((*id).clone());
            }
          }
        }
      },
      None => { return None; }
    };
    Some(result)
  }
}