import { Models } from '@rematch/core';
import { uiState } from './uiState';

export interface RootModel extends Models<RootModel> {
  uiState: typeof uiState;
}

export const models: RootModel = { uiState };
