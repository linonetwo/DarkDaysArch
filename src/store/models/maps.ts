/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { createModel } from '@rematch/core';
import type { RootModel } from './index';

import { CDDAMapgenWithCache, ItemId } from 'src/types/cdda/mapgen';

interface IMaps {
  activeOpenedMapIndex?: number;
  mapsInOpenedFile: CDDAMapgenWithCache['parsed_map'];
  /**
   * [width, height]
   * Each tileset have its own sprite_width sprite_height, but we want each tile width height to be constant in our map
   * 维护一个实际想要的大小，然后在 react 组件里根据 sprite_width sprite_height 和想要的大小的比值，把地图动态缩放一下
   */
  tileVisualWidthHeight: [number, number];
}

export interface IMapTileInfo {
  /**
   * [x, y]
   */
  position: [number, number];
  /**
   * [tileType, tileID] or [tileType, tileID][]
   */
  tiles: ItemId;
}

/**
 * 管理边栏UI的状态
 */
export const maps = createModel<RootModel>()({
  state: { mapsInOpenedFile: [], activeOpenedMapIndex: undefined, tileVisualWidthHeight: [50, 50] } as IMaps,
  reducers: {
    mapsInOpenedFileSetter(state, newMaps: CDDAMapgenWithCache['parsed_map']) {
      state.mapsInOpenedFile = newMaps;
      return state;
    },
    activeOpenedMapIndexSetter(state, activeOpenedMapIndex: number) {
      state.activeOpenedMapIndex = activeOpenedMapIndex;
      return state;
    },
  },
  selectors: (slice, createSelector, hasProps) => ({
    /**
     * Get map data for tiles to render
     */
    openedMapMatrix() {
      return slice((state): IMapTileInfo[][][] => {
        const openedMapTileIdMatrix = typeof state.activeOpenedMapIndex === 'number' ? state.mapsInOpenedFile[state.activeOpenedMapIndex] : [];
        const tileMatrixWithPosition: IMapTileInfo[][][] = openedMapTileIdMatrix.map((rows, rowIndex) =>
          rows.map((cell, columnIndex) => {
            return cell.map((cellItem) => {
              return {
                tiles: cellItem,
                position: [columnIndex * state.tileVisualWidthHeight[0], rowIndex * state.tileVisualWidthHeight[1]],
              };
            });
          }),
        );

        return tileMatrixWithPosition;
      });
    },
  }),
});
