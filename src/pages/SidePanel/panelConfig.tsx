import { ISidebarIconButton } from 'src/components/Sidebar/interface';
import { ISideControl } from 'src/components/SideControls/interface';
import { FileTree, OpenedFiles } from 'src/components/SideControls/controls';

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
      { icon: 'List', id: 'OpenedFiles', component: <OpenedFiles /> },
      { icon: 'AccountTree', id: 'FileTree', component: <FileTree /> },
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
      icon: 'SportsEsports',
      id: 'Simulation',
    },
    controls: [
      { icon: 'Subscriptions', id: 'SimulationParameter', component: <div>aaa</div> },
      { icon: 'SportsEsports', id: 'SimulationTools', component: <div>bbb</div> },
    ],
  },
];
