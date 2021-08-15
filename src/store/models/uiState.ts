/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { createModel } from '@rematch/core';
import type { RootModel } from './index';

interface IUIState {
  selectedButtonIndex: number;
  sideControlsCollapses: {
    [panelID: number]: Array<number | null>;
  };
  sideControlsSizes: {
    [panelID: number]: number[];
  };
  sidePanelWidth: number;
}

/**
 * 管理边栏UI的状态
 */
export const uiState = createModel<RootModel>()({
  state: { selectedButtonIndex: 0, sideControlsCollapses: {}, sideControlsSizes: {}, sidePanelWidth: 340 } as IUIState,
  reducers: {
    selectedButtonIndexSetter(state, selectedButtonIndex: number) {
      state.selectedButtonIndex = selectedButtonIndex;
      return state;
    },
    sideControlsCollapsesSetter(state, payload: { id: number; sizes: Array<number | null> }) {
      state.sideControlsCollapses[payload.id] = payload.sizes;
      return state;
    },
    sideControlsSizesSetter(state, payload: { id: number; sizes: number[] }) {
      state.sideControlsSizes[payload.id] = payload.sizes;
      return state;
    },
    sidePanelWidthSetter(state, sidePanelWidth: number) {
      state.sidePanelWidth = sidePanelWidth;
      return state;
    },
  },
});
