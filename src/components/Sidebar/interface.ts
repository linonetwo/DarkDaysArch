import * as MuiIcons from '@material-ui/icons';

export interface ISidebarIconButton {
  /** @material-ui/icons icon name */
  icon: keyof typeof MuiIcons;
  /** 作为定位功能的 id，同时也用于从 i18n 里取翻译的名字 */
  id: string;
  /** 快捷键，例如 VSCode 打开搜索的 shift+cmd+F */
  shortCut?: string;
}
