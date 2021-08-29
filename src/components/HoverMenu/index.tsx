/* eslint-disable unicorn/no-null */
import styled from 'styled-components';
import { Menu, MenuItem, IconName } from '@blueprintjs/core';
import { useSelector } from 'react-redux';
import { RootState } from 'src/store/store';

export interface IMenuItem {
  icon?: IconName;
  title: string;
  type: string;
}

const menuHeight = 200;
const menuWidth = 200;

const MenuContainer = styled.div`
  position: absolute;
  bottom: 0;
  right: 0;
  z-index: 100;
  max-width: ${menuWidth}px;

  transition: opacity 0.25s;

  ul.bp3-menu {
    max-height: ${menuHeight}px;
    overflow-y: auto;
  }
`;

export function HoverMenu(): JSX.Element | null {
  const [x, y] = useSelector((state: RootState) => [state.cameraMouse.mouseOnTileX, state.cameraMouse.mouseOnTileY]);
  const hoveredTiles = useSelector((state: RootState) => state.cameraMouse.hoveredTiles);
  if (hoveredTiles.length > 0) {
    return (
      <MenuContainer data-usage="hover-menu">
        <Menu>
          <MenuItem key="position" text={`(${x}, ${y})`} />
          {hoveredTiles.map((tileInfo) => {
            /** [tileType, tileID] */
            return <MenuItem key={tileInfo[1]} text={tileInfo[1]} />;
          })}
        </Menu>
      </MenuContainer>
    );
  }
  return null;
}
