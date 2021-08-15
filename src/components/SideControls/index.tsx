import { useTranslation } from 'react-i18next';
import styled from 'styled-components';
import { SplitPane } from 'react-collapse-pane';
import { ISideControl } from './interface';

const SideControlContainer = styled.div`
  height: 100%;
  width: 300px;

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
    <SplitPane split="horizontal">
      {props.controls.map((controlConfig) => (
        <SideControlContainer key={controlConfig.id}>
          <SideControlTitle>{t(`Icons.${controlConfig.id}`)}</SideControlTitle>
          {controlConfig.component}
        </SideControlContainer>
      ))}
    </SplitPane>
  );
}
