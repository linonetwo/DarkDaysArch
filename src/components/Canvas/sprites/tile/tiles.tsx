/* eslint-disable unicorn/no-null */
import { InteractionEvent } from 'pixi.js';
import { useDispatch, useSelector } from 'react-redux';
import { Dispatch, RootState, store } from 'src/store/store';
import { MapgenPaletteKeys } from 'src/types/cdda/mapgen';

import Tile from '.';

export default function Tiles(): JSX.Element {
  const openedMapMatrix = useSelector((state: RootState) => store.select.maps.openedMapMatrix(state));
  const dispatch = useDispatch<Dispatch>();

  return (
    <>
      {openedMapMatrix.map((rows) =>
        rows.map((cell) => {
          return cell.map((cellItem) => {
            const tilesInCell: Array<[MapgenPaletteKeys, string]> = cell.flatMap((cellItem) => {
              return [cellItem.tiles as [MapgenPaletteKeys, string]];
            });
            const [x, y] = cellItem.position;
            if (typeof cellItem.tiles[0] === 'string') {
              /** [tileType, tileID] */
              const tile = cellItem.tiles as [MapgenPaletteKeys, string];
              return (
                <Tile
                  x={x}
                  y={y}
                  tile={tile}
                  onHoverTile={(event: InteractionEvent) => {
                    dispatch.cameraMouse.hoverMouseOnTile({ tiles: tilesInCell, x, y });
                  }}
                />
              );
            }
            return null;
          });
        }),
      )}
    </>
  );
}
