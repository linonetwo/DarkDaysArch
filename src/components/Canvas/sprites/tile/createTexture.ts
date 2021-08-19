import { Texture, Rectangle, SCALE_MODES, BaseTexture } from 'pixi.js';

import { textureManager } from 'src/store/global/textureManager';
import { Direction, directionToIndex } from 'src/types/direction';
import { CDDATileSetConfig, ITileRandomSpriteDesc, ITileSetTile, ITileSetTilesNew, TileLayers } from 'src/types/tileset';

export interface ITileContext {
  direction?: Direction;
  tileSetData: CDDATileSetConfig;
  tileSubSetData: ITileSetTilesNew;
}

function getTileColumnRow(id: number, idStart: number, totalColumns: number): { column: number; row: number } {
  const actualID = id - idStart;
  const column = actualID % totalColumns;
  const row = Math.floor(actualID / totalColumns);
  return { column, row };
}

interface INewTileOptions {
  direction?: Direction;
  idStart: number;
  layer: TileLayers;
  tileHeight: number;
  tileID: number;
  tileName: string;
  tileWidth: number;
  totalColumns: number;
}

function createTileTextureFromBaseTexture(baseTexture: BaseTexture, newTileOptions: INewTileOptions): Texture {
  const { tileID, idStart, totalColumns, tileWidth, tileHeight, direction, layer, tileName } = newTileOptions;
  const xy = getTileColumnRow(tileID, idStart, totalColumns);
  // origin start at top-left, so going right-down, we have to provide negative value
  const newTileTexture = new Texture(baseTexture, new Rectangle(tileWidth * xy.column, tileHeight * xy.row, tileWidth, tileHeight));
  newTileTexture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
  textureManager.add(textureManager.getTileCacheID(tileName, layer, direction), newTileTexture);
  return newTileTexture;
}

function pickRandomTextureForTile(tileSpriteDesc: ITileRandomSpriteDesc): number {
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

export interface INewTileOptions1 {
  idStart: number;
  tileHeight: number;
  tileToRender: ITileSetTile | undefined;
  tileWidth: number;
  totalColumns: number;
}

export function getNewTileOptions(tileName: string, tileSetTexture: Texture, context: ITileContext): INewTileOptions1 {
  const { tileSetData, tileSubSetData } = context;
  const tileWidth = tileSubSetData.sprite_width ?? tileSetData.tile_info[0].width;
  const tileHeight = tileSubSetData.sprite_height ?? tileSetData.tile_info[0].height;
  // id can be string or array, array means ids in the array share the same texture
  const tileToRender = tileSubSetData.tiles.find((tile) => tile.id === tileName || (Array.isArray(tile.id) && tile.id.includes(tileName)));
  const totalColumns = tileSetTexture.orig.width / tileWidth;
  // we need to minus texture id with id start of this png, because tile-set put all ids of png in a tile_config.json, png in the later will have large id for its tiles
  const [, idStartString] = /range (\d+) to (\d+)/.exec(tileSubSetData?.['//'] ?? '') ?? [];
  const idStart = Number(idStartString);

  return {
    tileWidth,
    tileHeight,
    tileToRender,
    totalColumns,
    idStart,
  };
}

export function createTileTextures(
  tileName: string,
  tileSetTexture: Texture,
  layer: TileLayers,
  context: { direction?: Direction } & INewTileOptions1,
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
    if (typeof (tileIDs as number[] | ITileRandomSpriteDesc)[0] === 'number') {
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
    } else if (typeof (tileIDs as ITileRandomSpriteDesc)[0].spirit === 'number') {
      // or it can be single direction but randomly picked texture
      const index = pickRandomTextureForTile(tileIDs as ITileRandomSpriteDesc);
      newTileTexture = createTileTextureFromBaseTexture(tileSetTexture.baseTexture, {
        idStart,
        tileHeight,
        tileID: (tileIDs as ITileRandomSpriteDesc)[index].spirit,
        tileName,
        tileWidth,
        totalColumns,
        layer,
      });
    }
  }

  return newTileTexture!;
}
