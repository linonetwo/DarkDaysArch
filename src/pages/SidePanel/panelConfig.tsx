import { ISidebarIconButton } from 'src/components/Sidebar/interface';
import { ISideControl } from 'src/components/SideControls/interface';
import { FileTree, OpenedFiles } from 'src/components/SideControls/controls';
import { ReactChild, ReactFragment } from 'react';
import { SourceFileViewer } from 'src/components/Details';

export interface IPanelConfig {
  /** 左侧的竖着的按钮列表 */
  button: ISidebarIconButton;
  /** 中间的控制面板 */
  controls: ISideControl[];
  /** 详情界面 */
  detail: ReactChild | ReactFragment;
}

export const panelConfig: IPanelConfig[] = [
  {
    button: {
      icon: 'InsertDriveFile',
      id: 'File',
    },
    controls: [
      { icon: 'List', id: 'OpenedFiles', component: <OpenedFiles /> },
      { icon: 'AccountTree', id: 'FileTree', component: <FileTree /> },
    ],
    detail: <SourceFileViewer />,
  },
  {
    button: {
      icon: 'Add',
      id: 'Item',
    },
    controls: [
      { icon: 'History', id: 'ItemHistory', component: <div>aaa</div> },
      { icon: 'EmojiSymbols', id: 'ItemList', component: <div>bbb</div> },
    ],
    detail: <div>ccc</div>,
  },
  {
    button: {
      icon: 'Streetview',
      id: 'Layer',
    },
    controls: [
      { icon: 'NaturePeople', id: 'Layers', component: <div>aaa</div> },
      { icon: 'Accessible', id: 'LayerHelp', component: <div>bbb</div> },
    ],
    detail: <div>ccc</div>,
  },
  {
    button: {
      icon: 'Search',
      id: 'Search',
    },
    controls: [
      { icon: 'Search', id: 'Search', component: <div>aaa</div> },
      { icon: 'Help', id: 'SearchHelp', component: <div>bbb</div> },
    ],
    detail: <div>ccc</div>,
  },
  {
    button: {
      icon: 'SportsEsports',
      id: 'Simulation',
    },
    controls: [
      { icon: 'Subscriptions', id: 'SimulationParameter', component: <div>aaa</div> },
      { icon: 'SportsEsports', id: 'SimulationTools', component: <div>bbb</div> },
    ],
    detail: <div>ccc</div>,
  },
];
