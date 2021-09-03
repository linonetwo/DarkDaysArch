import { useState, useCallback, MouseEvent, useMemo, MutableRefObject, useEffect } from 'react';
import styled, { css } from 'styled-components';
import { Stage } from 'react-pixi-fiber';
import { Provider, useDispatch, useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';
import { Spector } from 'spectorjs';

import ContextMenu from '../ContextMenu';
import Tiles from './sprites/tile/tiles';
import { Direction } from 'src/store/models/cameraMouse';
import { Dispatch, RootState, store } from 'src/store/store';
import { RandomSpinner } from '../RandomSpinner';
import { useKeyPress } from './hooks';
import { HoverMenu } from '../HoverMenu';

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
    (keyPressSetReference: MutableRefObject<Set<string>>) => {
      if (keyPressSetReference.current.has('w') || keyPressSetReference.current.has('ArrowUp')) {
        // we have an inverse y-axis
        dispatch.cameraMouse.cameraMove({ directions: [Direction.down] });
      }
      if (keyPressSetReference.current.has('a') || keyPressSetReference.current.has('ArrowLeft')) {
        dispatch.cameraMouse.cameraMove({ directions: [Direction.left] });
      }
      if (keyPressSetReference.current.has('s') || keyPressSetReference.current.has('ArrowDown')) {
        dispatch.cameraMouse.cameraMove({ directions: [Direction.up] });
      }
      if (keyPressSetReference.current.has('d') || keyPressSetReference.current.has('ArrowRight')) {
        dispatch.cameraMouse.cameraMove({ directions: [Direction.right] });
      }
    },
    [dispatch],
  );
  const [downHandler, upHandler] = useKeyPress(handleKeyDownEvent);

  const sidePanelWidth = useSelector((state: RootState) => state.uiState.sidePanelWidth);
  const cameraPosition = useSelector((state: RootState) => ({ x: state.cameraMouse.cameraX, y: state.cameraMouse.cameraY }));
  const loadTexturesLoading = useSelector((state: RootState) => state.loading.effects.files.loadTextures);
  const actualWidth = useMemo(() => window.innerWidth - sidePanelWidth, [sidePanelWidth]);

  useEffect(() => {
    const spector = new Spector();
    spector.displayUI();
  }, []);

  if (loadTexturesLoading) {
    return (
      <Container sidePanelWidth={sidePanelWidth}>
        <div>{t('Loading')}</div>
        <RandomSpinner loading />
      </Container>
    );
  }

  return (
    <Container id={containerID} sidePanelWidth={sidePanelWidth} tabIndex={0} onKeyDown={downHandler} onKeyUp={upHandler}>
      <Stage
        // follow the camera
        pivot={cameraPosition}
        // center the camera
        position={{ x: 0, y: 0 }}
        options={{
          backgroundColor: 0x10_bb_99,
          height: window.innerHeight,
          width: actualWidth,
        }}
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
      <HoverMenu />
    </Container>
  );
}
