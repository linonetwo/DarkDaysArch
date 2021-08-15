import { ISidebarIconButton } from '../../components/Sidebar/interface';
import { ISideControl } from '../../components/SideControls/interface';

export interface IPanelConfig {
  button: ISidebarIconButton;
  controls: ISideControl[];
}

export const panelConfig: IPanelConfig[] = [
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
];
