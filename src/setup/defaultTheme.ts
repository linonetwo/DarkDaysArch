import { merge, cloneDeep } from 'lodash';
import { DefaultTheme } from 'styled-components';
import { createMuiTheme } from '@material-ui/core';

const position = {
  sideBarWidth: 40,
};

export const lightTheme: DefaultTheme = merge(cloneDeep(createMuiTheme()), {
  palette: {
    background: { default: 'rgb(250, 250, 250)' },
    border: { default: '1px solid #BBB' },
  },
  position,
});
export const darkTheme: DefaultTheme = merge(cloneDeep(createMuiTheme()), {
  palette: {
    background: { default: '#212121' },
    border: { default: '1px solid #666' },
  },
  position,
});
