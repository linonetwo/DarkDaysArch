import { Texture, Loader } from 'pixi.js';
import { useState, useLayoutEffect } from 'react';

import { textureManager, CDDATileLayers } from 'src/store/global/textureManager';
import { TILE_SET_CONFIG_FILE_NAME } from 'src/store/models/files';
import { CDDATileSetConfigWithCache } from 'src/types/cdda/tileset';
import { getNewTileOptions, createTileTextures } from './createTexture';

/**
 * 在 componentDidMount 的时候加载地块所需的贴图
 * @param baseTextureName 贴图所在的 png 的名字
 * @param tileName 地块的ID
 * @returns [fgTileTexture, bgTileTexture, tileWidthHeight]
 */
export function useTileTexture(tileName: string, tileVisualWidthHeight: [number, number]): [Texture | undefined, Texture | undefined, [number, number]] {
  const [fgTileTexture, fgTileTextureSetter] = useState<Texture | undefined>();
  const [bgTileTexture, bgTileTextureSetter] = useState<Texture | undefined>();
  // by default, this ratio (sprite_width / tileVisualWidthHeight[0].width) is 1, and we multiply it with visual width to get real width
  const [tileWidthHeight, tileWidthHeightSetter] = useState<[number, number]>([1 * tileVisualWidthHeight[0], 1 * tileVisualWidthHeight[1]]);

  useLayoutEffect(() => {
    // wait for texture to be generated from png image by PIXI loader
    Loader.shared.load((loader, resources) => {
      const tileSetData = resources[TILE_SET_CONFIG_FILE_NAME]?.data as CDDATileSetConfigWithCache['tile_data_index'] | undefined;
      if (tileSetData !== undefined) {
        const tileSubSetData = tileSetData[tileName];
        if (tileSubSetData !== undefined) {
          const { texture: tileSetTexture } = resources[tileSubSetData.tileset.file] ?? {};
          if (tileSetTexture !== undefined) {
            // TODO: calculate direction in rust
            let direction: undefined;
            // update width height with visual widthHeight
            const newTileOptions = getNewTileOptions(tileSetTexture, { tileSubSetData, direction, tileVisualWidthHeight });
            tileWidthHeightSetter([newTileOptions.tileVisualWidth, newTileOptions.tileVisualHeight]);

            // a tile can have foreground texture and a background texture
            fgTileTextureSetter(
              textureManager.getOrCreateTexture(textureManager.getTileCacheID(tileName, CDDATileLayers.fg, direction), () =>
                createTileTextures(tileName, tileSetTexture, CDDATileLayers.fg, {
                  direction,
                  ...newTileOptions,
                }),
              ),
            );
            bgTileTextureSetter(
              textureManager.getOrCreateTexture(textureManager.getTileCacheID(tileName, CDDATileLayers.bg, direction), () =>
                createTileTextures(tileName, tileSetTexture, CDDATileLayers.bg, {
                  direction,
                  ...newTileOptions,
                }),
              ),
            );
          }
        }
      }
    });
  }, []);

  return [fgTileTexture, bgTileTexture, tileWidthHeight];
}
