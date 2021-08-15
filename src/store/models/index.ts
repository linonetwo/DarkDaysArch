import { Models } from '@rematch/core';

import { cameraMouse } from './cameraMouse';
import { uiState } from './uiState';

export interface RootModel extends Models<RootModel> {
  cameraMouse: typeof cameraMouse;
  uiState: typeof uiState;
}

export const models: RootModel = { cameraMouse, uiState };
