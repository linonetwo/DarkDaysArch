/* eslint-disable no-param-reassign */
import { createModel } from '@rematch/core';
import { MapgenPaletteKeys } from 'src/types/cdda/mapgen';
import { RootModel } from './index';

export enum Direction {
  down = '↓',
  left = '←',
  right = '→',
  up = '↑',
}

const initialState = {
  cameraX: 0,
  cameraY: 0,
  mouseX: 0,
  mouseY: 0,
  /** Which tile is mouse on, x part */
  mouseOnTileX: 0,
  /** Which tile is mouse on, y part */
  mouseOnTileY: 0,
  hoveredTiles: [] as Array<[MapgenPaletteKeys, string]>,
};
type IState = typeof initialState;
export const cameraMouse = createModel<RootModel>()({
  state: initialState,
  reducers: {
    centerViewTo(
      state: IState,
      newLocation: {
        x: number;
        y: number;
      },
    ) {
      state.cameraX = newLocation.x;
      state.cameraY = newLocation.y;
      return state;
    },
    mouseMoveTo(
      state: IState,
      newLocation: {
        x: number;
        y: number;
      },
    ) {
      state.mouseX = newLocation.x;
      state.mouseY = newLocation.y;
      return state;
    },
    cameraMove(state: IState, payload: { directions: Direction[]; distance?: number }) {
      for (const direction of payload.directions) {
        switch (direction) {
          case Direction.up:
            state.cameraY += payload.distance ?? 10;
            break;
          case Direction.down:
            state.cameraY -= payload.distance ?? 10;
            break;
          case Direction.left:
            state.cameraX -= payload.distance ?? 10;
            break;
          case Direction.right:
            state.cameraX += payload.distance ?? 10;
            break;

          default:
            break;
        }
      }
      return state;
    },
    hoverMouseOnTile(
      state: IState,
      payload: { tile: [MapgenPaletteKeys, string]; x: number; y: number } | { tiles: Array<[MapgenPaletteKeys, string]>; x: number; y: number },
    ) {
      const { x, y } = payload;
      // only change when we move to a new tile
      if (x !== state.mouseOnTileX || y !== state.mouseOnTileY) {
        // in src/components/Canvas/sprites/tile/tiles.tsx we handle two case, one is that location only have one tile
        if ('tile' in payload) {
          const { tile } = payload;
          state.hoveredTiles = [tile];
        }
      } else if ('tiles' in payload) {
        // in src/components/Canvas/sprites/tile/tiles.tsx we handle two case, the other is that location have multiple overlapped tile, and only the one on the top can fire event to update this tiles
        const { tiles } = payload;
        state.hoveredTiles = tiles;
      }
      return state;
    },
  },
});
