/* eslint-disable @typescript-eslint/no-unsafe-return */
import { useCallback } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import styled from 'styled-components';

import { Dispatch, RootState } from 'src/store/store';
import { ISidebarIconButton } from './interface';
import { SidebarIconButton } from './SidebarIconButton';

const SidebarContainer = styled.div`
  height: 100%;
  width: ${(props) => props.theme.position.sideBarWidth}px;

  background-color: ${(props) => props.theme.palette.background.default};
  border-right: ${(props) => props.theme.palette.border.default};

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;

  /* react-collapse-pane will make SideControls position absolute so cover Sidebar, we have to make Sidebar absolute too to make it on the top */
  position: absolute;
  z-index: 2;
`;
const SidebarBottomItems = styled.div`
  align-self: flex-end;

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
`;

export interface ISidebarProps {
  bottomButtons?: ISidebarIconButton[];
  buttons: ISidebarIconButton[];
}

export function Sidebar(props: ISidebarProps): JSX.Element {
  const dispatch = useDispatch<Dispatch>();
  const selectedButtonIndexSetter = useCallback((nextButtonIndex: number) => dispatch.uiState.selectedButtonIndexSetter(nextButtonIndex), [dispatch.uiState]);
  const selectedButtonIndex = useSelector((state: RootState) => state.uiState.selectedButtonIndex);
  return (
    <SidebarContainer>
      {props.buttons.map((buttonConfig, index) => (
        <SidebarIconButton onClick={() => selectedButtonIndexSetter(index)} key={buttonConfig.id} {...buttonConfig} selected={selectedButtonIndex === index} />
      ))}

      <SidebarBottomItems>
        {props.bottomButtons?.map((buttonConfig, index) => (
          <SidebarIconButton
            onClick={() => selectedButtonIndexSetter(props.bottomButtons!.length + index)}
            key={buttonConfig.id}
            {...buttonConfig}
            selected={selectedButtonIndex === props.bottomButtons!.length + index}
          />
        ))}
      </SidebarBottomItems>
    </SidebarContainer>
  );
}
