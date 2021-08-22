import { useState, useCallback, MouseEvent, KeyboardEvent, useMemo } from 'react';
import styled, { css } from 'styled-components';
import { Stage } from 'react-pixi-fiber';
import { Provider, useDispatch, useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';

import ContextMenu from '../ContextMenu';
import Tiles from './sprites/tile/tiles';
import { Direction } from 'src/store/models/cameraMouse';
import { Dispatch, RootState, store } from 'src/store/store';
import { RandomSpinner } from '../RandomSpinner';

const Container = styled.main<{ sidePanelWidth: number }>`
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  ${({ sidePanelWidth }) => css`
    position: absolute;
    left: ${sidePanelWidth}px;
    width: calc(100vw - ${sidePanelWidth}px);
  `}
`;

const containerID = 'game-container';

export function World(): JSX.Element {
  const { t } = useTranslation();
  const dispatch = useDispatch<Dispatch>();
  const setMousePosition = useCallback(
    (event: MouseEvent<HTMLCanvasElement>) => {
      if (typeof event.clientX === 'number' && typeof event.clientY === 'number') {
        dispatch.cameraMouse.mouseMoveTo({ x: event.clientX, y: event.clientY });
      }
    },
    [dispatch],
  );
  const [contextMenuIsOpen, contextMenuIsOpenSetter] = useState(false);

  const handleKeyDownEvent = useCallback(
    (event: KeyboardEvent<HTMLCanvasElement>) => {
      switch (event.key) {
        case 'w':
          dispatch.cameraMouse.cameraMove({ direction: Direction.up });
          break;

        default:
          break;
      }
    },
    [dispatch],
  );

  const sidePanelWidth = useSelector((state: RootState) => state.uiState.sidePanelWidth);
  const loadTexturesLoading = useSelector((state: RootState) => state.loading.effects.files.loadTextures);
  const actualWidth = useMemo(() => window.innerWidth - sidePanelWidth, [sidePanelWidth]);

  if (loadTexturesLoading) {
    return (
      <Container sidePanelWidth={sidePanelWidth}>
        <div>{t('Loading')}</div>
        <RandomSpinner loading />
      </Container>
    );
  }

  return (
    <Container id={containerID} sidePanelWidth={sidePanelWidth}>
      <Stage
        // follow the camera
        pivot={{
          x: 0,
          y: 0,
        }}
        // center the camera
        position={{ x: actualWidth / 2, y: window.innerHeight / 2 }}
        options={{
          backgroundColor: 0x10_bb_99,
          height: window.innerHeight,
          width: actualWidth,
        }}
        onKeyDown={handleKeyDownEvent}
        onMouseMove={setMousePosition}
        onContextMenu={(event: MouseEvent<HTMLCanvasElement>) => {
          event.preventDefault();
          // reopen the menu to refresh its props
          contextMenuIsOpenSetter(false);
          setImmediate(() => {
            contextMenuIsOpenSetter(true);
          });
        }}
        onClick={() => {
          contextMenuIsOpenSetter(false);
        }}>
        <Provider store={store}>
          <Tiles />
        </Provider>
      </Stage>

      <ContextMenu
        items={[
          {
            title: 'entity.name',
            type: 'entity[]',
            icon: 'people',
          },
        ]}
        open={contextMenuIsOpen}
        mountPoint={containerID}
      />
    </Container>
  );
}
