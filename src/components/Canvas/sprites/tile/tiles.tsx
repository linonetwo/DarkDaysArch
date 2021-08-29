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
              if (Array.isArray(cellItem.tiles[0])) {
                return cellItem.tiles as Array<[MapgenPaletteKeys, string]>;
              } else {
                return [cellItem.tiles as [MapgenPaletteKeys, string]];
              }
            });
            const [x, y] = cellItem.position;
            if (Array.isArray(cellItem.tiles[0])) {
              /** [tileType, tileID][] */
              const tiles = cellItem.tiles as Array<[MapgenPaletteKeys, string]>;
              return tiles.map((tile) => {
                return (
                  <Tile
                    key={`${x}-${y}-${tile[1]}`}
                    x={x}
                    y={y}
                    tile={tile}
                    onHoverTile={(event: InteractionEvent) => {
                      dispatch.cameraMouse.hoverMouseOnTile({ tiles: tilesInCell, x, y });
                    }}
                  />
                );
              });
            } else if (typeof cellItem.tiles[0] === 'string') {
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
