// styled.d.ts
import { Theme } from 'styled-components';

declare module 'styled-components' {
  export interface DefaultTheme extends Theme {
    palette: {
      background: {
        default: string;
      };
      border: {
        default: string;
      };
    };
    position: {
      sideBarWidth: number;
    };
  }
}
