import React, { useEffect, useState } from 'react';
import { Sprite, Container, Text } from 'react-pixi-fiber';
import { Loader, Texture, Point, Rectangle, SCALE_MODES } from 'pixi.js';

import { CDDATileSetConfig } from 'src/types/tileset';

const centerAnchor = new Point(0.5, 0.5);

export interface IPawnProps {
  textureName: string;
  x: number;
  y: number;
}

function getTileColumnRow(id: number, idStart: number, totalColumns: number): { column: number; row: number } {
  const actualID = id - idStart;
  const column = actualID % totalColumns;
  const row = Math.floor(actualID / totalColumns);
  return { column, row };
}

export default React.memo(function Tile(props: IPawnProps): JSX.Element {
  const [fgTileTexture, fgTileTextureSetter] = useState<Texture | undefined>();
  const [bgTileTexture, bgTileTextureSetter] = useState<Texture | undefined>();
  const [tileWidthHeight, tileWidthHeightSetter] = useState<[number, number]>([100, 100]);
  const tileID = 'ranch_camp_17';
  useEffect(() => {
    Loader.shared.load((loader, resources) => {
      const { texture: tileSetTexture } = resources[props.textureName] ?? {};
      if (tileSetTexture !== undefined) {
        const tileSetData = resources['assets/ChibiUltica/tile_config.json']?.data as CDDATileSetConfig | undefined;
        if (tileSetData !== undefined) {
          const tileSubSetData = tileSetData['tiles-new'].find((item) => props.textureName.endsWith(item.file));
          if (tileSubSetData !== undefined) {
            const tileWidth = tileSubSetData.sprite_width ?? tileSetData.tile_info[0].width;
            const tileHeight = tileSubSetData.sprite_height ?? tileSetData.tile_info[0].height;
            tileWidthHeightSetter([tileWidth, tileHeight]);
            const tileToRender = tileSubSetData.tiles.find((tile) => tile.id === tileID);
            const totalColumns = tileSetTexture.orig.width / tileWidth;
            const [, idStartString] = /range (\d+) to (\d+)/.exec(tileSubSetData?.['//'] ?? '') ?? [];
            const idStart = Number(idStartString);
            let bgColumn = 0;
            let bgRow = 0;
            let fgColumn = 0;
            let fgRow = 0;
            if (typeof tileToRender?.bg === 'number') {
              const xy = getTileColumnRow(tileToRender.bg, idStart, totalColumns);
              bgColumn = xy.column;
              bgRow = xy.row;
            }
            if (typeof tileToRender?.fg === 'number') {
              const xy = getTileColumnRow(tileToRender.fg, idStart, totalColumns);
              fgColumn = xy.column;
              fgRow = xy.row;
            }
            // origin start at top-left, so going right-down, we have to provide negative value
            const fgX = tileWidth * bgColumn;
            const fgY = tileHeight * bgRow;
            const bgX = tileWidth * fgColumn;
            const bgY = tileHeight * fgRow;
            // TODO: init texture in a pool
            const newFgTileTexture = new Texture(tileSetTexture.baseTexture, new Rectangle(fgX, fgY, tileWidth, tileHeight));
            const newBgTileTexture = new Texture(tileSetTexture.baseTexture, new Rectangle(bgX, bgY, tileWidth, tileHeight));
            newFgTileTexture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
            newBgTileTexture.baseTexture.scaleMode = SCALE_MODES.NEAREST;
            fgTileTextureSetter(newFgTileTexture);
            bgTileTextureSetter(newBgTileTexture);
          }
        }
      }
    });
  }, []);
  if (fgTileTexture === undefined && bgTileTexture === undefined) return <Text text={`No Tile Texture "${props.textureName}"`} x={0} y={0} />;

  return (
    <Container>
      <Sprite
        width={tileWidthHeight[0]}
        height={tileWidthHeight[1]}
        anchor={centerAnchor}
        interactive
        x={props.x}
        y={props.y}
        cursor="pointer"
        texture={fgTileTexture}
      />
      <Sprite
        width={tileWidthHeight[0]}
        height={tileWidthHeight[1]}
        anchor={centerAnchor}
        interactive
        x={props.x}
        y={props.y}
        cursor="pointer"
        texture={bgTileTexture}
      />
    </Container>
  );
});
