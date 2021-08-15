/* eslint-disable import/namespace */
import { useTranslation } from 'react-i18next';
import styled, { css } from 'styled-components';
import { Tooltip } from '@material-ui/core';
import * as MuiIcons from '@material-ui/icons';

import { ISidebarIconButton } from './interface';
import { createElement } from 'react';

const SidebarIconButtonContainer = styled.div<{ selected: boolean }>`
  width: ${(props) => props.theme.position.sideBarWidth}px;
  height: ${(props) => props.theme.position.sideBarWidth}px;

  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  opacity: 0.6;

  ${({ selected }) =>
    selected
      ? css`
          border-left: solid 1px #333;
          opacity: 0.8;
        `
      : css`
          /** border will move icon to right by 1px, so we pad other icons by 1px to make sure they align together */
          margin-left: 1px;
        `}

  &:hover {
    opacity: 1;
    cursor: pointer;
  }
`;

export interface ISidebarIconButtonProps extends ISidebarIconButton {
  onClick: () => void;
  selected: boolean;
}

export function SidebarIconButton(props: ISidebarIconButtonProps): JSX.Element {
  const { t } = useTranslation();
  return (
    <Tooltip title={t(`Icons.${props.id}`) ?? props.id} placement="right">
      <SidebarIconButtonContainer onClick={props.onClick} selected={props.selected}>
        {createElement(MuiIcons[props.icon])}
      </SidebarIconButtonContainer>
    </Tooltip>
  );
}
