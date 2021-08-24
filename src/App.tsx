import { useEffect } from 'react';
import styled from 'styled-components';
import { useDispatch, useSelector } from 'react-redux';

import './globalStyles';

import { Dispatch, RootState } from 'src/store/store';
import { SidePanel } from './pages/SidePanel';
import { panelConfig } from './pages/SidePanel/panelConfig';

const AppContainer = styled.div`
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  display: flex;
  flex-direction: row;
`;
const DetailContainer = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
`;

export default function App(): JSX.Element {
  const dispatch = useDispatch<Dispatch>();
  // TODO: debug code, load example file
  useEffect(() => {
    void dispatch.files.popDialogAndLoadFileTree();
    void dispatch.files.loadFile('json/magic_academy.json').then(() => {
      dispatch.files.selectFile('json/magic_academy.json');
    });
    void dispatch.files.loadTextures('assets/ChibiUltica');
  }, []);

  const selectedButtonIndex = useSelector((state: RootState) => state.uiState.selectedButtonIndex);

  return (
    <AppContainer>
      <SidePanel />
      <DetailContainer>{panelConfig[selectedButtonIndex]?.detail}</DetailContainer>
    </AppContainer>
  );
}
