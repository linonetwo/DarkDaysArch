import { useEffect } from 'react';
import styled from 'styled-components';
import { useDispatch, useSelector } from 'react-redux';
import { Loader } from 'pixi.js';
import { invoke } from '@tauri-apps/api';

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
    void dispatch.files.loadFile('magic_academy.json').then(() => {
      dispatch.files.selectFile('magic_academy.json');
    });
  }, []);
  useEffect(() => {
    // prevent resource is already loaded when dev hot reload
    try {
      Loader.shared.add('assets/ChibiUltica/normal.png');
      Loader.shared.add('assets/ChibiUltica/tile_config.json');
    } catch {}
  }, []);
  useEffect(() => {
    // prevent Unhandled Rejection (TypeError): window.rpc is undefined when open in browser
    try {
      void invoke('read_tileset_folder', { pathName: '/Users/linonetwo/Desktop/repo/DarkDaysArch/public/assets/ChibiUltica' }).then(console.log);
    } catch {}
  });
  const selectedButtonIndex = useSelector((state: RootState) => state.uiState.selectedButtonIndex);

  return (
    <AppContainer>
      <SidePanel />
      <DetailContainer>{panelConfig[selectedButtonIndex]?.detail}</DetailContainer>
    </AppContainer>
  );
}
