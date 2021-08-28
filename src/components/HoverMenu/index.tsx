/* eslint-disable unicorn/no-null */
import styled from 'styled-components';
import { Menu, MenuItem, IconName } from '@blueprintjs/core';
import { useSelector } from 'react-redux';
import { RootState, store } from 'src/store/store';
import { IMapTileInfo } from 'src/store/models/maps';
import { MapgenPaletteKeys } from 'src/types/cdda/mapgen';

export interface IMenuItem {
  icon?: IconName;
  title: string;
  type: string;
}

const menuHeight = 200;
const menuWidth = 200;
interface MenuContainerProps {
  left?: number;
  top?: number;
}
const MenuContainer = styled.div<MenuContainerProps>`
  position: absolute;
  top: ${({ top }) => (top ?? -10_000) + 20}px;
  left: calc(${({ left }) => left ?? '-10000'}px - ${menuWidth / 2}px);
  z-index: 100;
  max-width: ${menuWidth}px;

  transition: opacity 0.25s;

  ul.bp3-menu {
    max-height: ${menuHeight}px;
    overflow-y: auto;
  }
`;

export function HoverMenu(): JSX.Element | null {
  const sidePanelWidth = useSelector((state: RootState) => state.uiState.sidePanelWidth);
  const openedMapMatrix = useSelector((state: RootState) => store.select.maps.openedMapMatrix(state));
  const mousePosition = useSelector((state: RootState) => ({ x: state.cameraMouse.mouseX - sidePanelWidth, y: state.cameraMouse.mouseY }));
  const hoveredTiles: IMapTileInfo[] = [];
  openedMapMatrix.forEach((rows) =>
    rows.forEach((cell) => {
      return cell.forEach((cellItem) => {
        const [x, y] = cellItem.position;
        if (mousePosition.x > x && mousePosition.x < x + 50 && mousePosition.y > y && mousePosition.y < y + 50) {
          hoveredTiles.push(cellItem);
        }
      });
    }),
  );
  if (hoveredTiles.length > 0) {
    return (
      <MenuContainer data-usage="hover-menu" top={mousePosition.y} left={mousePosition.x}>
        <Menu>
          {hoveredTiles.map((cellItem) => {
            if (Array.isArray(cellItem.tiles[0])) {
              /** [tileType, tileID][] */
              const tiles = cellItem.tiles as Array<[MapgenPaletteKeys, string]>;
              return tiles.map((tile) => {
                return <MenuItem key={tile[1]} text={tile[1]} />;
              });
            } else if (typeof cellItem.tiles[0] === 'string') {
              /** [tileType, tileID] */
              const tile = cellItem.tiles as [MapgenPaletteKeys, string];
              return <MenuItem key={tile[1]} text={tile[1]} />;
            }
            return null;
          })}
        </Menu>
      </MenuContainer>
    );
  }
  return null;
}
