import { useState, useCallback, MouseEvent, KeyboardEvent, useMemo } from 'react';
import styled from 'styled-components';
import { Stage } from 'react-pixi-fiber';
import { Provider, useDispatch, useSelector } from 'react-redux';
import ContextMenu from '../ContextMenu';
import Tiles from './sprites/tile/tiles';
import { Direction } from 'src/store/models/cameraMouse';
import { Dispatch, RootState, store } from 'src/store/store';

const Container = styled.main``;

const containerID = 'game-container';

export function World(): JSX.Element {
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
  const actualWidth = useMemo(() => window.innerWidth - sidePanelWidth, [sidePanelWidth]);

  return (
    <Container id={containerID}>
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
