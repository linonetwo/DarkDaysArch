import React, { useEffect, useState } from 'react';
import { TilingSprite, Container, Text } from 'react-pixi-fiber';
import { Loader, Texture, Point } from 'pixi.js';

const tileWidthHeight = 50;

const centerAnchor = new Point(0.5, 0.5);

export interface IPawnProps {
  textureName: string;
  x: number;
  y: number;
}

export default React.memo(function Tile(props: IPawnProps): JSX.Element {
  const [tileTexture, tileTextureSetter] = useState<Texture | undefined>();
  useEffect(() => {
    Loader.shared.load((loader, resources) => {
      const { texture } = resources[props.textureName];
      if (texture !== undefined) {
        tileTextureSetter(texture);
      }
    });
  });
  if (tileTexture === undefined) return <Text text={`No Tile Texture "${props.textureName}"`} x={0} y={0} />;

  return (
    <Container>
      <TilingSprite width={tileWidthHeight} height={tileWidthHeight} anchor={centerAnchor} x={props.x} y={props.y} texture={tileTexture} />
    </Container>
  );
});
