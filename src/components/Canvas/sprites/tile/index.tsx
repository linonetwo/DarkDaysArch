import { memo } from 'react';
import { TilingSprite, Container, Text } from 'react-pixi-fiber';
import { Point } from 'pixi.js';

import { useTileTexture } from './useTileTexture';

const centerAnchor = new Point(0.5, 0.5);

export interface ITileProps {
  tileName: string;
  x: number;
  y: number;
}

export default memo(function Tile(props: ITileProps): JSX.Element {
  const { tileName, x, y } = props;
  const [fgTileTexture, bgTileTexture, tileWidthHeight] = useTileTexture(tileName);
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
