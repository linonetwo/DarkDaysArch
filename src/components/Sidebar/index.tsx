import styled from 'styled-components';
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
  onClickButton: (index: number) => void;
  selectedButtonIndex: number;
}

export function Sidebar(props: ISidebarProps): JSX.Element {
  return (
    <SidebarContainer>
      {props.buttons.map((buttonConfig, index) => (
        <SidebarIconButton onClick={() => props.onClickButton(index)} key={buttonConfig.id} {...buttonConfig} selected={props.selectedButtonIndex === index} />
      ))}

      <SidebarBottomItems>
        {props.bottomButtons?.map((buttonConfig, index) => (
          <SidebarIconButton
            onClick={() => props.onClickButton(props.bottomButtons!.length + index)}
            key={buttonConfig.id}
            {...buttonConfig}
            selected={props.selectedButtonIndex === props.bottomButtons!.length + index}
          />
        ))}
      </SidebarBottomItems>
    </SidebarContainer>
  );
}
