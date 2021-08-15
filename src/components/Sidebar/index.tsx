import styled from 'styled-components';
import { ISidebarIconButton } from './interface';
import { SidebarIconButton } from './SidebarIconButton';

const SidebarContainer = styled.div`
  height: 100%;
  width: 40px;

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
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
}

export function Sidebar(props: ISidebarProps): JSX.Element {
  return (
    <SidebarContainer>
      {props.buttons.map((buttonConfig, index) => (
        <SidebarIconButton onClick={() => props.onClickButton(index)} key={buttonConfig.id} {...buttonConfig} />
      ))}

      <SidebarBottomItems>
        {props.bottomButtons?.map((buttonConfig, index) => (
          <SidebarIconButton onClick={() => props.onClickButton(props.bottomButtons!.length + index)} key={buttonConfig.id} {...buttonConfig} />
        ))}
      </SidebarBottomItems>
    </SidebarContainer>
  );
}
