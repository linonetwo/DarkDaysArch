import { memo, useCallback } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { Sprite, Container, Text } from 'react-pixi-fiber';
import { Point, InteractionEvent, TextStyle } from 'pixi.js';

import { useTileTexture } from './useTileTexture';
import { Dispatch, RootState } from 'src/store/store';
import { MapgenPaletteKeys } from 'src/types/cdda/mapgen';

const centerAnchor = new Point(0.5, 0.5);
const defaultTextStyle = new TextStyle({
  align: 'center',
  fontFamily: '"Source Sans Pro", Helvetica, sans-serif',
  fontSize: 6,
  fontWeight: '300',
  letterSpacing: 1,
  wordWrap: true,
  wordWrapWidth: 50,
});

export interface ITileProps {
  onHoverTile?: (event: InteractionEvent) => void;
  tile: [MapgenPaletteKeys, string];
  x: number;
  y: number;
}

export default memo(function Tile(props: ITileProps): JSX.Element {
  const {
    tile: [tileType, tileID],
    x,
    y,
    onHoverTile,
  } = props;
  const tileVisualWidthHeight = useSelector((state: RootState) => state.maps.tileVisualWidthHeight);
  const [fgTileTexture, bgTileTexture, [width, height]] = useTileTexture(tileID, tileVisualWidthHeight);

  if (fgTileTexture === undefined && bgTileTexture === undefined)
    return (
      <Text
        text={`No Tile Texture "${tileID}" of "${tileType}"`}
        x={x}
        y={y}
        anchor={centerAnchor}
        style={defaultTextStyle}
        mouseover={onHoverTile}
        interactive
      />
    );

  return (
    <Container interactive cursor="pointer" mouseover={onHoverTile}>
      <Sprite width={width} height={height} anchor={centerAnchor} x={x} y={y} texture={bgTileTexture} />
      <Sprite width={width} height={height} anchor={centerAnchor} x={x} y={y} texture={fgTileTexture} />
    </Container>
  );
});
