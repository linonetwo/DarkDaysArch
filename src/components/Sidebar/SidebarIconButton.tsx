/* eslint-disable import/namespace */
import { useTranslation } from 'react-i18next';
import styled from 'styled-components';
import { Tooltip } from '@material-ui/core';
import * as MuiIcons from '@material-ui/icons';

import { ISidebarIconButton } from './interface';
import { createElement } from 'react';

const SidebarIconButtonContainer = styled.div`
  width: 100%;
  height: 100%;

  &:after {
    content: '';
    display: block;
    padding-bottom: 100%;
  }
`;

export interface ISidebarIconButtonProps extends ISidebarIconButton {
  onClick: () => void;
}

export function SidebarIconButton(props: ISidebarIconButtonProps): JSX.Element {
  const { t } = useTranslation();
  return (
    <Tooltip title={t(`Icons.${props.id}`) ?? props.id} placement="right">
      <SidebarIconButtonContainer onClick={props.onClick}>{createElement(MuiIcons[props.icon])}</SidebarIconButtonContainer>
    </Tooltip>
  );
}
