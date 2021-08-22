export const TILE_SET_CONFIG_FILE_NAME = 'tile_config.json';
/**
 * @docs https://github.com/CleverRaven/Cataclysm-DDA/blob/master/doc/TILESET.md
 */
export interface CDDATileSetConfig {
  tile_info: ITileSetTileInfo[];
  'tiles-new': ITileSetTilesNew[];
}

/**
 * provided by rust side
 * @link src-tauri/src/types/tileset.rs
 */
export interface CDDATileSetConfigWithCache {
  raw_config: CDDATileSetConfig;
  /**
   * tileset in base64 format
   */
  textures: Record<string, string>;
}

interface ITileSetTileInfo {
  height: number;
  pixelscale: number;
  width: number;
}

export type TileLayers = 'fg' | 'bg';

export type ITileRandomSpriteDesc = Array<{ spirit: number; weight: number }>;
type imageID = number | number[] | ITileRandomSpriteDesc;

export interface ITileSetTile {
  '//'?: string;
  additional_tiles?: ITileSetAdditionalTile[];
  animated?: boolean;
  bg: imageID;
  fg: imageID;
  id: string | string[];
  multitile?: boolean;
  rotates?: boolean;
}

export interface ITileSetTilesNew {
  '//'?: string;
  ascii?: ITileSetAscii[];
  file: string;
  sprite_height?: number;
  sprite_offset_x?: number;
  sprite_offset_y?: number;
  sprite_width?: number;
  tiles: ITileSetTile[];
}

interface ITileSetAdditionalTile {
  bg: imageID;
  fg: imageID;
  id: string;
}

interface ITileSetAscii {
  bold: boolean;
  color: string;
  offset: number;
}
