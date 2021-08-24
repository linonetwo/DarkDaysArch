import { Models } from '@rematch/core';

import { cameraMouse } from './cameraMouse';
import { files } from './files';
import { maps } from './maps';
import { uiState } from './uiState';

export interface RootModel extends Models<RootModel> {
  cameraMouse: typeof cameraMouse;
  files: typeof files;
  maps: typeof maps;
  uiState: typeof uiState;
}

export const models: RootModel = { cameraMouse, files, maps, uiState };
