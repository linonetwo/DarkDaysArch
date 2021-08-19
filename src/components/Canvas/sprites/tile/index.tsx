import React, { useEffect, useState } from 'react';
import { TilingSprite, Container, Text } from 'react-pixi-fiber';
import { Loader, Texture, Point, Rectangle, SCALE_MODES } from 'pixi.js';

import { CDDATileSetConfig, ITileSetTilesNew } from 'src/types/tileset';
import { textureManager } from 'src/store/global/textureManager';
import { createTileTextures, getNewTileOptions } from './createTexture';

const centerAnchor = new Point(0.5, 0.5);

export interface ITileProps {
  tileName: string;
  x: number;
  y: number;
}

const textureName = 'assets/ChibiUltica/normal.png';
export default React.memo(function Tile(props: ITileProps): JSX.Element {
  const { tileName, x, y } = props;
  const [fgTileTexture, fgTileTextureSetter] = useState<Texture | undefined>();
  const [bgTileTexture, bgTileTextureSetter] = useState<Texture | undefined>();
  const [tileWidthHeight, tileWidthHeightSetter] = useState<[number, number]>([100, 100]);

  useEffect(() => {
    Loader.shared.load((loader, resources) => {
      const { texture: tileSetTexture } = resources[textureName] ?? {};
      if (tileSetTexture !== undefined) {
        const tileSetData = resources['assets/ChibiUltica/tile_config.json']?.data as CDDATileSetConfig | undefined;
        if (tileSetData !== undefined) {
          const tileSubSetData = tileSetData['tiles-new'].find((item) => textureName.endsWith(item.file));
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
  if (fgTileTexture === undefined && bgTileTexture === undefined) return <Text text={`No Tile Texture "${tileName}"`} x={0} y={0} />;

  return (
    <Container>
      <TilingSprite
        width={tileWidthHeight[0]}
        height={tileWidthHeight[1]}
        anchor={centerAnchor}
        interactive
        x={x}
        y={y}
        cursor="pointer"
        texture={bgTileTexture}
      />
      <TilingSprite
        width={tileWidthHeight[0]}
        height={tileWidthHeight[1]}
        anchor={centerAnchor}
        interactive
        x={x}
        y={y}
        cursor="pointer"
        texture={fgTileTexture}
      />
    </Container>
  );
});
