import { useState } from 'react';
import styled from 'styled-components';
import { Resizable } from 're-resizable';

import { Sidebar } from '../../components/Sidebar';
import { SideControls } from '../../components/SideControls';
import { panelConfig } from './panelConfig';

const SidePanelContainer = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
`;

export function SidePanel(): JSX.Element {
  const [width, widthSetter] = useState(340);
  const [selectedButtonIndex, selectedButtonIndexSetter] = useState(0);

  return (
    <Resizable
      size={{ width, height: '100%' }}
      onResizeStop={(_, direction, __, d) => {
        widthSetter(width + d.width);
      }}>
      <SidePanelContainer>
        <SideControls controls={panelConfig[selectedButtonIndex]?.controls ?? []} />
        <Sidebar buttons={panelConfig.map(({ button }) => button)} onClickButton={selectedButtonIndexSetter} />
      </SidePanelContainer>
    </Resizable>
  );
}
