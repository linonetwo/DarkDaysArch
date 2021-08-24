/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { createModel } from '@rematch/core';
import type { RootModel } from './index';

import mapgen from 'src/types/cdda/mapgen';

interface IMaps {
  activeOpenedMapIndex?: number;
  mapsInOpenedFile: mapgen.CDDAMapgenWithCache['parsedMap'];
}

/**
 * 管理边栏UI的状态
 */
export const maps = createModel<RootModel>()({
  state: { mapsInOpenedFile: [], activeOpenedMapIndex: undefined } as IMaps,
  reducers: {
    mapsInOpenedFileSetter(state, newMaps: mapgen.CDDAMapgenWithCache['parsedMap']) {
      state.mapsInOpenedFile = newMaps;
      return state;
    },
    activeOpenedMapIndexSetter(state, activeOpenedMapIndex: number) {
      state.activeOpenedMapIndex = activeOpenedMapIndex;
      return state;
    },
  },
});
