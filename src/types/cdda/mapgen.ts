/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

/**
 * A char in map rows can mean multiple item, like # mean a terrain and a furniture, and some terrain can have id same as a furniture, so we have to keep id's type in a tuple
 */
export type ItemIDOrItemList = [MapgenPaletteKeys, string] | [MapgenPaletteKeys, string][];
export type MapgenPaletteKeys = "terrain" | "furniture";
export type CDDAMapgenFurniture = string | string[];
export type CDDAMapgenItem = string | CDDAMapgenItemRandomListItem | CDDAMapgenItemRandomListItem[];
export type CDDAMapgenTerrain = string | CDDAMapgenTerrainRandomListItem[];
export type CDDAMapgenTerrainRandomListItem = string | [string, number];
export type CDDAMapgenTrap = string | CDDAMapgenTrapObject;

export interface CDDAMapgenWithCache {
  /**
   * Map 2D array that have place-holder characters replaced with actual item ID, for map view to display And we have multiple mapgen in a file, so this will be a 3D matrix. But each location can have terrain, furniture, item and so on, so each tile will be a list, so this is a 4D tensor
   */
  parsedMap: ItemIDOrItemList[][][][];
  /**
   * Full mapgen file content, for code editor to display
   */
  rawMapgen: CDDAMapgen[];
  [k: string]: unknown;
}
export interface CDDAMapgen {
  method: string;
  object: CDDAMapgenObject;
  om_terrain: string;
  type: string;
  [k: string]: unknown;
}
export interface CDDAMapgenObject {
  fill_ter: string;
  /**
   * @example `{ "=": "f_magic_bench", "-": "f_alembic", "?": "f_rack_wood" }`
   */
  furniture?: {
    [k: string]: CDDAMapgenFurniture;
  };
  items?: {
    [k: string]: CDDAMapgenItem;
  };
  palettes?: string[];
  place_loot?: CDDAMapgenPlaceLoot[];
  place_monsters?: CDDAMapgenPlaceMonster[];
  rows: string[];
  /**
   * @example `{ "_": "t_open_air", ")": "t_wall_glass", "#": "t_rock_wall", "-": "t_floor", "]": "t_door_glass_c" }`
   */
  terrain: {
    [k: string]: CDDAMapgenTerrain;
  };
  /**
   * @example `{ "=": "tr_rollmat" }`
   */
  traps?: {
    [k: string]: CDDAMapgenTrap;
  };
  [k: string]: unknown;
}
export interface CDDAMapgenItemRandomListItem {
  chance: number;
  item: string;
  repeat?: number[];
  [k: string]: unknown;
}
export interface CDDAMapgenPlaceLoot {
  chance: number;
  item: string;
  x: number;
  y: number;
  [k: string]: unknown;
}
export interface CDDAMapgenPlaceMonster {
  density: number;
  monster: string;
  repeat: number[];
  x: number;
  y: number;
  [k: string]: unknown;
}
export interface CDDAMapgenTrapObject {
  trap: string;
  [k: string]: unknown;
}
