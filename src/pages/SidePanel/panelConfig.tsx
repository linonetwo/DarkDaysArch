import { ISidebarIconButton } from '../../components/Sidebar/interface';
import { ISideControl } from '../../components/SideControls/interface';

export interface IPanelConfig {
  button: ISidebarIconButton;
  controls: ISideControl[];
}

export const panelConfig: IPanelConfig[] = [
  {
    button: {
      icon: 'InsertDriveFile',
      id: 'File',
    },
    controls: [
      { icon: 'List', id: 'OpenedFiles', component: <div>aaa</div> },
      { icon: 'AccountTree', id: 'FileTree', component: <div>bbb</div> },
    ],
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
  },
  {
    button: {
      icon: 'Games',
      id: 'Simulation',
    },
    controls: [
      { icon: 'Search', id: 'Search', component: <div>aaa</div> },
      { icon: 'Help', id: 'SearchHelp', component: <div>bbb</div> },
    ],
  },
];
