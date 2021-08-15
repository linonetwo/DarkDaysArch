/* eslint-disable @typescript-eslint/no-unsafe-return */
import { useTranslation } from 'react-i18next';
import styled from 'styled-components';
import { SplitPane } from 'react-collapse-pane';
import { ISideControl } from './interface';
import { Button } from '@material-ui/core';
import { useDispatch, useSelector } from 'react-redux';
import { Dispatch, RootState } from 'src/store/store';

const SideControlContainer = styled.div`
  height: 100%;
  /* react-collapse-pane will make SideControls position absolute so cover Sidebar, we have to add a margin left so they don't overlap */
  margin-left: ${(props) => props.theme.position.sideBarWidth}px;
  /** and we delete that margin from width (100% of absolute react-collapse-pane) */
  width: calc(100% - ${(props) => props.theme.position.sideBarWidth}px);

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
  const sideControlsSizes = useSelector((state: RootState) => state.uiState.sideControlsSizes[state.uiState.selectedButtonIndex]);
  const sideControlsCollapses = useSelector((state: RootState) => state.uiState.sideControlsCollapses[state.uiState.selectedButtonIndex]);
  const selectedButtonIndex = useSelector((state: RootState) => state.uiState.selectedButtonIndex);
  const dispatch = useDispatch<Dispatch>();
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
      }}
      hooks={{
        onSaveSizes: (sizes: number[]) => dispatch.uiState.sideControlsSizesSetter({ id: selectedButtonIndex, sizes }),
        onCollapse: (sizes: Array<number | null>) => dispatch.uiState.sideControlsCollapsesSetter({ id: selectedButtonIndex, sizes }),
      }}
      initialSizes={sideControlsSizes}
      collapsedSizes={sideControlsCollapses}>
      {props.controls.map((controlConfig) => (
        <SideControlContainer key={controlConfig.id}>
          <SideControlTitle>{t(`Controls.${controlConfig.id}`)}</SideControlTitle>
          {controlConfig.component}
        </SideControlContainer>
      ))}
    </SplitPane>
  );
}
