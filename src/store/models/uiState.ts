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

const arrayEquals = (a?: Array<number | null>, b?: Array<number | null>): boolean => {
  if ((a === undefined && b !== undefined) || (a !== undefined && b === undefined)) {
    return false;
  }
  if (a === undefined && b === undefined) {
    return true;
  }
  let equal = a?.length === b?.length;
  if (!equal) return false;
  a!.forEach((item, index) => {
    equal = equal && item === b![index];
  });
  return equal;
};
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
      if (!arrayEquals(state.sideControlsCollapses[payload.id], payload.sizes)) {
        state.sideControlsCollapses[payload.id] = payload.sizes;
      }
      return state;
    },
    sideControlsSizesSetter(state, payload: { id: number; sizes: number[] }) {
      if (!arrayEquals(state.sideControlsSizes[payload.id], payload.sizes)) {
        state.sideControlsSizes[payload.id] = payload.sizes;
      }
      return state;
    },
    sidePanelWidthSetter(state, sidePanelWidth: number) {
      state.sidePanelWidth = sidePanelWidth;
      return state;
    },
  },
});
