import { Texture, Rectangle, SCALE_MODES, BaseTexture } from 'pixi.js';

import { CDDATileLayers, textureManager } from 'src/store/global/textureManager';
import { Direction, directionToIndex } from 'src/types/direction';
import { CDDATileSetInverseIndexedTileData, CDDATileSetRandomSpriteDescItem, CDDATileSetTile } from 'src/types/cdda/tileset';

export interface ITileContext {
  direction?: Direction;
  tileSubSetData: CDDATileSetInverseIndexedTileData;
  tileVisualWidthHeight: [number, number];
}

/**
 * Calculate single tile texture column and row in the tileset large png texture
 * @param id
 * @param idStart
 * @param totalColumns
 * @returns
 */
function getTileColumnRow(id: number, idStart: number, totalColumns: number): { column: number; row: number } {
  const actualID = id - idStart;
  const column = actualID % totalColumns;
  const row = Math.floor(actualID / totalColumns);
  return { column, row };
}

interface INewTileOptions {
  direction?: Direction;
  idStart: number;
  layer: CDDATileLayers;
  tileHeight: number;
  tileID: number;
  tileName: string;
  tileWidth: number;
  totalColumns: number;
}

/**
 * Get single tile texture from tileset large png texture
 * @param baseTexture
 * @param newTileOptions
 * @returns
 */
function createTileTextureFromBaseTexture(baseTexture: BaseTexture, newTileOptions: INewTileOptions): Texture {
  const { tileID, idStart, totalColumns, tileWidth, tileHeight, direction, layer, tileName } = newTileOptions;
  const xy = getTileColumnRow(tileID, idStart, totalColumns);
  // origin start at top-left, so going right-down, we have to provide negative value
  const newTileTexture = new Texture(baseTexture, new Rectangle(tileWidth * xy.column, tileHeight * xy.row, tileWidth, tileHeight));
  newTileTexture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  textureManager.add(textureManager.getTileCacheID(tileName, layer, direction), newTileTexture);
  return newTileTexture;
}

/**
 * Pick random sprite for one of the CDDA tile config structure
 * @param tileSpriteDesc
 * @returns
 */
function pickRandomTextureForTile(tileSpriteDesc: CDDATileSetRandomSpriteDescItem[]): number {
  const weightCount = tileSpriteDesc.reduce((previous, current) => previous + current.weight, 0);
  let randomSpriteIndex = Math.floor(Math.random() * weightCount);
  for (const [index, element] of tileSpriteDesc.entries()) {
    randomSpriteIndex -= element.weight;
    if (randomSpriteIndex <= 0) {
      return index;
    }
  }
  return tileSpriteDesc.length - 1;
}

export interface ICreateTileOptions {
  idStart: number;
  tileHeight: number;
  tileToRender: CDDATileSetTile | undefined;
  tileVisualHeight: number;
  tileVisualWidth: number;
  tileWidth: number;
  totalColumns: number;
}

/**
 * Get options for `createTileTextures`
 * @param tileName
 * @param tileSetTexture
 * @param context
 * @returns
 */
export function getNewTileOptions(tileSetTexture: Texture, context: ITileContext): ICreateTileOptions {
  const { tileSubSetData, tileVisualWidthHeight } = context;
  const {
    sprite_width: tileWidth,
    sprite_height: tileHeight,
    sprite_height_ratio: tileHeightRatio,
    sprite_width_ratio: tileWidthRatio,
  } = tileSubSetData.tileset;
  if (typeof tileWidth !== 'number') {
    throw new TypeError(`tileWidth is ${typeof tileWidth} in getNewTileOptions, ${JSON.stringify(context)}`);
  }
  if (typeof tileHeight !== 'number') {
    throw new TypeError(`tileHeight is ${typeof tileHeight} in getNewTileOptions, ${JSON.stringify(context)}`);
  }
  if (typeof tileWidthRatio !== 'number') {
    throw new TypeError(`tileWidth is ${typeof tileWidth} in getNewTileOptions, ${JSON.stringify(context)}`);
  }
  if (typeof tileHeightRatio !== 'number') {
    throw new TypeError(`tileHeight is ${typeof tileHeight} in getNewTileOptions, ${JSON.stringify(context)}`);
  }
  // id can be string or array, array means ids in the array share the same texture
  const tileToRender = tileSubSetData.tile;
  const totalColumns = tileSetTexture.orig.width / tileWidth;
  // we need to minus texture id with id start of this png, because tile-set put all ids of png in a tile_config.json, png in the later will have large id for its tiles
  const idStart = tileSubSetData.start_id;

  return {
    tileWidth,
    tileHeight,
    tileVisualWidth: tileWidthRatio * tileVisualWidthHeight[0],
    tileVisualHeight: tileHeightRatio * tileVisualWidthHeight[1],
    tileToRender,
    totalColumns,
    idStart,
  };
}

/**
 * call `createTileTextureFromBaseTexture`, create texture in 3 different ways, adapt CDDA 's tileset JSON structure
 * @param tileName
 * @param tileSetTexture
 * @param layer
 * @param context
 * @returns
 */
export function createTileTextures(
  tileName: string,
  tileSetTexture: Texture,
  layer: CDDATileLayers,
  context: { direction?: Direction } & ICreateTileOptions,
): Texture {
  const { direction, tileWidth, tileHeight, tileToRender, totalColumns, idStart } = context;

  let newTileTexture: undefined | Texture;
  // bg can be a number id (that should minus the start id of this png)
  if (typeof tileToRender?.[layer] === 'number') {
    newTileTexture = createTileTextureFromBaseTexture(tileSetTexture.baseTexture, {
      idStart,
      tileHeight,
      tileID: tileToRender[layer] as number,
      tileName,
      tileWidth,
      totalColumns,
      layer,
    });
  }
  // can be an array indicate ←→ or ↑→↓← variant of this tile
  // "fg": ["mon_dog_left", "mon_dog_right"] or "bg": ["t_wall_n", "t_wall_e", "t_wall_s", "t_wall_w"]
  if (tileToRender !== undefined && Array.isArray(tileToRender[layer])) {
    const tileDirectionIndex = directionToIndex[direction ?? Direction.up];
    const tileIDs = tileToRender[layer];
    if (typeof (tileIDs as number[] | CDDATileSetRandomSpriteDescItem[])[0] === 'number') {
      newTileTexture = createTileTextureFromBaseTexture(tileSetTexture.baseTexture, {
        idStart,
        tileHeight,
        tileID: (tileIDs as number[])[tileDirectionIndex],
        tileName,
        tileWidth,
        totalColumns,
        layer,
        // with direction option
        direction,
      });
    } else if (typeof (tileIDs as CDDATileSetRandomSpriteDescItem[])[0].spirit === 'number') {
      // or it can be single direction but randomly picked texture
      const index = pickRandomTextureForTile(tileIDs as CDDATileSetRandomSpriteDescItem[]);
      newTileTexture = createTileTextureFromBaseTexture(tileSetTexture.baseTexture, {
        idStart,
        tileHeight,
        tileID: (tileIDs as CDDATileSetRandomSpriteDescItem[])[index].sprite,
        tileName,
        tileWidth,
        totalColumns,
        layer,
      });
    }
  }

  return newTileTexture!;
}
