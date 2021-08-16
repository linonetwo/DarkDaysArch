import { Models } from '@rematch/core';

import { cameraMouse } from './cameraMouse';
import { files } from './files';
import { uiState } from './uiState';

export interface RootModel extends Models<RootModel> {
  cameraMouse: typeof cameraMouse;
  files: typeof files;
  uiState: typeof uiState;
}

export const models: RootModel = { cameraMouse, files, uiState };
