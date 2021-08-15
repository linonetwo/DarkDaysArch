import { ReactChild, ReactFragment } from 'react';
import * as MuiIcons from '@material-ui/icons';

export interface ISideControl {
  /** 展示在可折叠区域内的组件 */
  component: ReactChild | ReactFragment;
  /** @material-ui/icons icon name */
  icon: keyof typeof MuiIcons;
  /** 作为定位功能的 id，同时也用于从 i18n 里取翻译的名字 */
  id: string;
}
