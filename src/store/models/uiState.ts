import { createModel } from '@rematch/core';
import type { RootModel } from './index';

/**
 * 管理边栏UI的状态
 */
export const uiState = createModel<RootModel>()({
  state: { selectedButtonIndex: 0, sidePanelWidth: 340 },
  reducers: {
    sidePanelWidthSetter(state, sidePanelWidth: number) {
      state.sidePanelWidth = sidePanelWidth;
      return state;
    },
    selectedButtonIndexSetter(state, selectedButtonIndex: number) {
      state.selectedButtonIndex = selectedButtonIndex;
      return state;
    },
  },
});
