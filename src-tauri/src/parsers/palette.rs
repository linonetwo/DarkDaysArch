use data::types::{mapgen, palette};

use crate::utils;

pub fn lookup_mapgen_char_in_palette(character: &char, palette: &palette::CDDAPalette) -> Vec<mapgen::ItemIDOrItemList> {
  let char_string = character.to_string();
  let mut items_this_tile: Vec<mapgen::ItemIDOrItemList> = vec![];
  // each type may have some different logic, so we cannot abstract these

  // terrain
  let terrain_value_option = palette.mapping_object.terrain.get(&char_string);
  match terrain_value_option {
    Some(terrain_value) => match terrain_value {
      // "a": "t_thconc_floor",
      palette::CDDAPaletteTerrainValue::Id(id) => {
        items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, id.clone())));
      }
      palette::CDDAPaletteTerrainValue::Object(terrain_value) => {
        items_this_tile.push(mapgen::ItemIDOrItemList::Id((
          mapgen::MapgenPaletteKeys::terrain,
          terrain_value.terrain.clone(),
        )));
      }
      // "o": [["t_window_domestic", 10], "t_window_no_curtains", "t_window_open", "t_window_no_curtains_open", ["t_curtains", 5]],
      // possible: [["t_window_domestic", 10], ["t_window_no_curtains", "t_window_open"], "t_window_no_curtains_open", [["t_curtains", 5], ["t_door_o", 5], "t_door_locked_interior"]
      palette::CDDAPaletteTerrainValue::RandomList(random_list_ids) => {
        let random_id = pick_random_list_id_by_distribution(&random_list_ids);
        match random_id {
          Some(id) => {
            items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, id)));
          }
          None => {}
        };
      }
      palette::CDDAPaletteTerrainValue::ParamRef(ref_object) => {
        let reference_parameter_option = &palette.parameters.get(&ref_object.param);
        match reference_parameter_option {
          Some(palette_parameter) => {
            // TODO: cache the choose, as https://github.com/CleverRaven/Cataclysm-DDA/blob/b3c2331b4788cf77cf3edd83e51f9434a8a73789/doc/MAPGEN.md#mapgen-parameters said this choice should be consistent during a mapgen
            let random_id = pick_random_list_id_by_distribution(&palette_parameter.default.distribution);
            match random_id {
              Some(id) => {
                items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, id)));
              }
              None => {}
            };
          }
          None => {
            items_this_tile.push(mapgen::ItemIDOrItemList::Id((mapgen::MapgenPaletteKeys::terrain, ref_object.fallback.clone())));
          }
        };
      }
    },
    None => {}
  };
  // furniture
  // let furniture_value_option = palette.furniture.get(&char_string);
  // matchfurniture_value_option {
  //   Some(furniture) => {
  //     let furniture_value_option = furniture.get(&char_string);
  //     match furniture_value_option {
  //       Some(furniture_value) => match furniture_value {
  //         palette::CDDAPaletteFurnitureValue::Id(id) => id.clone(),
  //       },
  //     }
  //   }
  // }
  items_this_tile
}

pub fn pick_random_list_id_by_distribution(random_list_ids: &palette::CDDAPaletteDistribution) -> Option<String> {
  let mut random_list_item_picker = utils::random_id::RandomList { ..Default::default() };
  match random_list_ids {
    palette::CDDAPaletteDistribution::Id(id) => {
      random_list_item_picker.add(id, 1);
    }
    palette::CDDAPaletteDistribution::IdList(id_list) => {
      for id in id_list {
        random_list_item_picker.add(id, 1);
      }
    }
    palette::CDDAPaletteDistribution::IdWithWeight(id, weight) => {
      random_list_item_picker.add(id, *weight);
    }
    palette::CDDAPaletteDistribution::RecursiveMixed(id_or_id_with_weight_list) => {
      for id_or_id_with_weight in id_or_id_with_weight_list {
        match id_or_id_with_weight {
          palette::CDDAPaletteDistributionMixed::Id(id) => {
            random_list_item_picker.add(id, 1);
          }
          palette::CDDAPaletteDistributionMixed::IdWithWeight(id, weight) => {
            random_list_item_picker.add(id, *weight);
          }
        }
      }
    }
  };
  // get random one from the list
  let result = random_list_item_picker.random_get();
  match result {
    Some(id) => Some((*id).clone()),
    None => None,
  }
}
