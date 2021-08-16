/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { createModel } from '@rematch/core';
import type { RootModel } from './index';

export interface IFileTree {
  /** 文件夹的子文件 */
  children?: IFileTree[];
  /** 自动生成的内容简介 */
  description?: string;
  /** 文件或文件夹的完整路径 */
  path: string;
}
export interface IOpenedFiles {
  /**
   * 文件内容，可以是字符串或者 JSON 对象等
   */
  content: string | any;
  path: string;
}
export enum GameFileType {
  mod,
  image,
  others,
}

interface IMapsState {
  /** 当前选中在编辑的文件 */
  activeOpenedFile?: string;
  /** 打开的几个工作区，可以查看里面的文件内容 */
  fileTrees: IFileTree[];
  openedFiles: IOpenedFiles[];
}

/**
 * 管理当前打开的地图文件、Mod资源等
 */
export const files = createModel<RootModel>()({
  state: { activeOpenedFile: undefined, fileTrees: [], openedFiles: [] } as IMapsState,
  reducers: {
    selectFile(state, selectedFilePath?: string) {
      state.activeOpenedFile = selectedFilePath;
      return state;
    },
    addNewFileTree(state, newFileTree: IFileTree) {
      // existed, update it
      const existedIndex = state.fileTrees.findIndex((fileTree) => fileTree.path === newFileTree.path);
      if (existedIndex >= 0) {
        state.fileTrees[existedIndex] = newFileTree;
      } else {
        // new one, push to list
        state.fileTrees.push(newFileTree);
      }
      return state;
    },
    removeFileTree(state, fileTreeIndex: number) {
      state.fileTrees.splice(fileTreeIndex, 1);
      return state;
    },
    addNewOpenedFiles(state, newOpenedFile: IOpenedFiles) {
      const existedIndex = state.openedFiles.findIndex((openedFiles) => openedFiles.path === newOpenedFile.path);
      // existed, update it
      if (existedIndex >= 0) {
        state.openedFiles[existedIndex] = newOpenedFile;
      } else {
        // new one, push to list
        state.openedFiles.push(newOpenedFile);
      }
      return state;
    },
    removeOpenedFiles(state, openedFileIndex: number) {
      state.openedFiles.splice(openedFileIndex, 1);
      return state;
    },
  },
  effects: (dispatch) => ({
    async popDialogAndLoadFileTree() {
      try {
        // eslint-disable-next-line @typescript-eslint/await-thenable
        await 1;
        // TODO: call tauri dialog api
        dispatch.files.addNewFileTree({ path: 'aaa' });
      } catch (error) {
        console.error(error);
      }
    },
    async loadFile(filePath: string) {
      try {
        // eslint-disable-next-line @typescript-eslint/await-thenable
        const content = await import('./magic_academy.json');
        const newFile = { path: filePath, content };
        // TODO: call tauri dialog api
        dispatch.files.addNewOpenedFiles(newFile);
      } catch (error) {
        console.error(error);
      }
    },
  }),
});
