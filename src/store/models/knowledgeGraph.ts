/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { createModel } from '@rematch/core';
import type { RootModel } from './index';

import { ArrayOf_CDDATerrain } from 'src/types/cdda/terrain';

interface IKnowledgeGraph {
  terrain: ArrayOf_CDDATerrain;
}

/**
 * 管理边栏UI的状态
 */
export const knowledgeGraph = createModel<RootModel>()({
  state: { terrain: [] } as IKnowledgeGraph,
  reducers: {
    update(state, newKnowledge: Partial<IKnowledgeGraph>) {
      if (Array.isArray(newKnowledge.terrain)) {
        state.terrain = newKnowledge.terrain;
      }
      return state;
    },
  },
});
