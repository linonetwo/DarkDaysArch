/* eslint-disable unicorn/no-null */
import { useSelector } from 'react-redux';
import { RootState } from 'src/store/store';

import Tile from '.';

export default function Tiles(): JSX.Element {
  const openedMapMatrix = useSelector((state: RootState) =>
    typeof state.maps.activeOpenedMapIndex === 'number' ? state.maps.mapsInOpenedFile[state.maps.activeOpenedMapIndex] : [],
  );

  return (
    <>
      {openedMapMatrix.map((rows, rowIndex) =>
        rows.map((cell, columnIndex) => {
          return cell.map((cellItem) => {
            if (Array.isArray(cellItem)) {
              const [idType, id] = cellItem;
              if (typeof id === 'string') {
                return <Tile x={columnIndex * 30} y={rowIndex * 30} tileName={id} />;
              }
            }
            return null;
          });
        }),
      )}
    </>
  );
}
