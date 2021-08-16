/* eslint-disable @typescript-eslint/no-unsafe-return */
import { useCallback, useState } from 'react';
import styled from 'styled-components';
import { Resizable } from 're-resizable';
import { useSelector, useDispatch } from 'react-redux';

import { Sidebar } from '../../components/Sidebar';
import { SideControls } from '../../components/SideControls';
import { panelConfig } from './panelConfig';
import { Dispatch, RootState } from '../../store/store';

const Container = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
`;
const SidePanelContainer = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
`;
const DetailContainer = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
`;

export function SidePanel(): JSX.Element {
  const dispatch = useDispatch<Dispatch>();
  const width = useSelector((state: RootState) => state.uiState.sidePanelWidth);
  const selectedButtonIndex = useSelector((state: RootState) => state.uiState.selectedButtonIndex);

  return (
    <Container>
      <Resizable
        size={{ width, height: '100%' }}
        onResizeStop={(_, direction, __, d) => {
          dispatch.uiState.sidePanelWidthSetter(width + d.width);
        }}>
        <SidePanelContainer>
          {panelConfig.map(({ controls }, index) => (
            <SideControls key={index} controls={controls ?? []} id={index} />
          ))}
          <Sidebar buttons={panelConfig.map(({ button }) => button)} />
        </SidePanelContainer>
      </Resizable>
      <DetailContainer>{panelConfig[selectedButtonIndex]?.detail}</DetailContainer>
    </Container>
  );
}
