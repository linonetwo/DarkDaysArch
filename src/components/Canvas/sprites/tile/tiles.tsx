import Tile from '.';

export default function Tiles(): JSX.Element {
  // const tiles = useMappedQuery(positions, (props: MappedComponentProps<[typeof VisibleItem, typeof TileComponent]>) => {
  //   const { x, y, texture } = props.components[0];
  //   return <Tile x={x} y={y} textureName={texture} />;
  // });
  const tiles = [null];

  return <>{tiles}</>;
}
