import Tile from '.';

export default function Tiles(): JSX.Element {
  // const tiles = useMappedQuery(positions, (props: MappedComponentProps<[typeof VisibleItem, typeof TileComponent]>) => {
  //   const { x, y, texture } = props.components[0];
  //   return <Tile x={x} y={y} textureName={texture} />;
  // });

  return (
    <>
      <Tile x={100} y={0} tileName={'ranch_camp_17'} />
    </>
  );
}
