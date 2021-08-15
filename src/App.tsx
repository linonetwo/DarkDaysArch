import styled from 'styled-components';

import { SidePanel } from './pages/SidePanel';

const AppContainer = styled.div`
  width: 100vw;
  height: 100vh;
  overflow: hidden;
`;

export default function App(): JSX.Element {
  return (
    <AppContainer>
      <SidePanel />
    </AppContainer>
  );
}
