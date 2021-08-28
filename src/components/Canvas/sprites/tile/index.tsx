import { memo, useMemo } from 'react';
import { useSelector } from 'react-redux';
import { Sprite, Container, Text } from 'react-pixi-fiber';
import { Point } from 'pixi.js';

import { useTileTexture } from './useTileTexture';
import { RootState } from 'src/store/store';

const centerAnchor = new Point(0.5, 0.5);

export interface ITileProps {
  tileName: string;
  x: number;
  y: number;
}

export default memo(function Tile(props: ITileProps): JSX.Element {
  const { tileName, x, y } = props;
  const tileVisualWidthHeight = useSelector((state: RootState) => state.maps.tileVisualWidthHeight);
  const [fgTileTexture, bgTileTexture, [width, height]] = useTileTexture(tileName, tileVisualWidthHeight);
  if (fgTileTexture === undefined && bgTileTexture === undefined) return <Text text={`No Tile Texture "${tileName}"`} x={x} y={y} />;

  return (
    <Container>
      <Sprite width={width} height={height} anchor={centerAnchor} interactive x={x} y={y} cursor="pointer" texture={bgTileTexture} />
      <Sprite width={width} height={height} anchor={centerAnchor} interactive x={x} y={y} cursor="pointer" texture={fgTileTexture} />
    </Container>
  );
});
