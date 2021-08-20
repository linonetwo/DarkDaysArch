import { Texture, Loader } from 'pixi.js';
import { useState, useLayoutEffect } from 'react';

import { textureManager } from 'src/store/global/textureManager';
import { CDDATileSetConfig } from 'src/types/tileset';
import { getNewTileOptions, createTileTextures } from './createTexture';

/**
 * 在 componentDidMount 的时候加载地块所需的贴图
 * @param baseTextureName 贴图所在的 png 的名字
 * @param tileName 地块的ID
 * @returns [fgTileTexture, bgTileTexture, tileWidthHeight]
 */
export function useTileTexture(baseTextureName: string, tileName: string): [Texture | undefined, Texture | undefined, [number, number]] {
  const [fgTileTexture, fgTileTextureSetter] = useState<Texture | undefined>();
  const [bgTileTexture, bgTileTextureSetter] = useState<Texture | undefined>();
  const [tileWidthHeight, tileWidthHeightSetter] = useState<[number, number]>([100, 100]);

  useLayoutEffect(() => {
    Loader.shared.load((loader, resources) => {
      const { texture: tileSetTexture } = resources[baseTextureName] ?? {};
      if (tileSetTexture !== undefined) {
        const tileSetData = resources['assets/ChibiUltica/tile_config.json']?.data as CDDATileSetConfig | undefined;
        if (tileSetData !== undefined) {
          const tileSubSetData = tileSetData['tiles-new'].find((item) => baseTextureName.endsWith(item.file));
          if (tileSubSetData !== undefined) {
            // TODO: calculate direction in rust
            let direction: undefined;

            const newTileOptions = getNewTileOptions(tileName, tileSetTexture, { tileSetData, tileSubSetData, direction });
            tileWidthHeightSetter([newTileOptions.tileWidth, newTileOptions.tileHeight]);

            // a tile can have foreground texture and a background texture
            fgTileTextureSetter(
              textureManager.getOrCreateTexture(textureManager.getTileCacheID(tileName, 'fg', direction), () =>
                createTileTextures(tileName, tileSetTexture, 'fg', {
                  direction,
                  ...newTileOptions,
                }),
              ),
            );
            bgTileTextureSetter(
              textureManager.getOrCreateTexture(textureManager.getTileCacheID(tileName, 'bg', direction), () =>
                createTileTextures(tileName, tileSetTexture, 'bg', {
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
