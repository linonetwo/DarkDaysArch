/* eslint-disable @typescript-eslint/consistent-type-assertions */
import { createModel } from '@rematch/core';
import { Loader, LoaderResource } from 'pixi.js';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';

import type { RootModel } from './index';
import { CDDATileSetConfigWithCache } from 'src/types/cdda/tileset';
import { CDDAMapgenWithCache } from 'src/types/cdda/mapgen';
import { ModLoadingEventPayload } from 'src/types/cdda/tauri_event';

export const TILE_SET_CONFIG_FILE_NAME = 'tile_config.json';

export interface IFileTree {
  /** 文件夹的子文件 */
  children?: IFileTree[];
  /** 自动生成的内容简介 */
  description?: string;
  /** 文件或文件夹的完整路径 */
  path: string;
}
export interface IOpenedFile {
  /**
   * 文件内容，可以是字符串或者 JSON 对象等
   */
  content: string;
  path: string;
}
export enum GameFileType {
  mod,
  image,
  others,
}

interface IMapsState {
  /** 当前选中在编辑的文件 */
  activeOpenedFilePath?: string;
  /** 打开的几个工作区，可以查看里面的文件内容 */
  fileTrees: IFileTree[];
  /** 文件加载进度信息 */
  loadingProgress: {
    /** 所有本次要加载的文件列表 */
    fileList: ModLoadingEventPayload['fileList'];
    /** 已加载完几个 */
    loadedCount: number;
    loading: boolean;
  };
  /** 当前在编辑器（内存）里打开的文件 */
  openedFiles: IOpenedFile[];
}

/**
 * 管理当前打开的地图文件、Mod资源等
 */
export const files = createModel<RootModel>()({
  state: { activeOpenedFilePath: undefined, fileTrees: [], loadingProgress: { loading: false, fileList: [], loadedCount: 0 }, openedFiles: [] } as IMapsState,
  reducers: {
    selectFile(state, selectedFilePath?: string) {
      state.activeOpenedFilePath = selectedFilePath;
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
    addNewOpenedFiles(state, newOpenedFile: IOpenedFile) {
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
    updateLoadingProgress(state, newProgress: IMapsState['loadingProgress']) {
      state.loadingProgress = newProgress;
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
    async loadFile(mapgenFilePath: string) {
      try {
        // eslint-disable-next-line @typescript-eslint/await-thenable
        const mapgenWithCache = await invoke<CDDAMapgenWithCache>('read_mapgen_file', {
          mapgenFilePath,
        });
        const newFile = { path: mapgenFilePath, content: JSON.stringify(mapgenWithCache.raw_mapgen, undefined, '  ') };
        dispatch.files.addNewOpenedFiles(newFile);
        // load map to display
        dispatch.maps.mapsInOpenedFileSetter(mapgenWithCache.parsed_map);
        dispatch.maps.activeOpenedMapIndexSetter(0);
      } catch (error) {
        console.error(error);
      }
    },
    async loadMods(dataFolderPath: string) {
      try {
        // eslint-disable-next-line @typescript-eslint/await-thenable
        await invoke<string>('load_cdda_data_folder', {
          dataFolderPath,
        });
        // dispatch.knowledgeGraph.update({ terrain });
      } catch (error) {
        console.error(error);
      }
    },
    async listenModLoadingProgress() {
      const unlisten = await listen<ModLoadingEventPayload>('game://mod-loading', (event) => {
        const { payload } = event;
        const { fileList } = payload;
        const total = fileList.reduce((previousCount, file) => previousCount + file.file_count, 0);
        const loadedCount = fileList.reduce((previousCount, file) => previousCount + file.progress, 0);
        dispatch.files.updateLoadingProgress({
          loading: true,
          fileList,
          loadedCount,
        });
      });
    },
    async loadTextures(tilesetPathName: string) {
      // prevent Unhandled Rejection (TypeError): window.rpc is undefined when open in browser
      try {
        const { tile_data_index: tileDataIndex, textures } = await invoke<CDDATileSetConfigWithCache>('read_tileset_folder', {
          tilesetPathName,
        });
        const tileConfigResource = new LoaderResource(TILE_SET_CONFIG_FILE_NAME, TILE_SET_CONFIG_FILE_NAME);
        tileConfigResource.data = tileDataIndex;
        Loader.shared.resources[tileConfigResource.name] = tileConfigResource;
        for (const [tilesetName, tilesetImage] of Object.entries(textures)) {
          Loader.shared.add(tilesetName, tilesetImage);
        }
      } catch (error) {
        console.log('loadTileConfigs error', error);
      }
    },
  }),
});
