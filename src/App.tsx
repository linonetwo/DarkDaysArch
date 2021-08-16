import { useEffect } from 'react';
import styled from 'styled-components';
import { useDispatch } from 'react-redux';

import './globalStyles';

import { Dispatch } from 'src/store/store';
import { SidePanel } from './pages/SidePanel';

const AppContainer = styled.div`
  width: 100vw;
  height: 100vh;
  overflow: hidden;
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
  return (
    <AppContainer>
      <SidePanel />
    </AppContainer>
  );
}
