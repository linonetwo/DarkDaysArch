import { useTranslation } from 'react-i18next';
import styled from 'styled-components';
import { SplitPane } from 'react-collapse-pane';
import { ISideControl } from './interface';
import { Button } from '@material-ui/core';

const SideControlContainer = styled.div`
  height: 100%;
  width: 300px;

  /* react-collapse-pane will make SideControls position absolute so cover Sidebar, we have to add a margin left so they don't overlap */
  margin-left: ${(props) => props.theme.position.sideBarWidth}px;

  background-color: ${(props) => props.theme.palette.background.default};
  border-right: ${(props) => props.theme.palette.border.default};

  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
`;
const SideControlTitle = styled.h4``;

export interface ISideControlsProps {
  controls: ISideControl[];
}

export function SideControls(props: ISideControlsProps): JSX.Element {
  const { t } = useTranslation();
  return (
    <SplitPane
      split="horizontal"
      collapse={{
        beforeToggleButton: <Button>↑</Button>,
        afterToggleButton: <Button>↓</Button>,
        overlayCss: {},
        buttonTransition: 'zoom',
        buttonPositionOffset: -20,
        collapsedSize: 50,
        collapseTransitionTimeout: 350,
      }}>
      {props.controls.map((controlConfig) => (
        <SideControlContainer key={controlConfig.id}>
          <SideControlTitle>{t(`Controls.${controlConfig.id}`)}</SideControlTitle>
          {controlConfig.component}
        </SideControlContainer>
      ))}
    </SplitPane>
  );
}
