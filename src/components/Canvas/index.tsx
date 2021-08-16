import React, { useState, useCallback } from 'react';
import styled from 'styled-components';
import { Stage } from 'react-pixi-fiber';
import { Provider, useDispatch } from 'react-redux';
import ContextMenu from '../ContextMenu';
import Tiles from './sprites/tile/tiles';
import { Direction } from 'src/store/models/cameraMouse';
import { Dispatch, store } from 'src/store/store';

const Container = styled.main``;

const containerID = 'game-container';

export default function World(): JSX.Element {
  const dispatch = useDispatch<Dispatch>();
  const setMousePosition = useCallback(
    (event: React.MouseEvent<HTMLCanvasElement, MouseEvent>) => {
      if (typeof event.clientX === 'number' && typeof event.clientY === 'number') {
        dispatch.cameraMouse.mouseMoveTo({ x: event.clientX, y: event.clientY });
      }
    },
    [dispatch],
  );
  const [contextMenuIsOpen, contextMenuIsOpenSetter] = useState(false);

  const handleKeyDownEvent = useCallback(
    (event: React.KeyboardEvent<HTMLCanvasElement>) => {
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

  return (
    <Container id={containerID}>
      <Stage
        // follow the camera
        pivot={{
          x: 0,
          y: 0,
        }}
        // center the camera
        position={{ x: window.innerWidth / 2, y: window.innerHeight / 2 }}
        options={{
          backgroundColor: 0x10_bb_99,
          height: window.innerHeight,
          width: window.innerWidth,
        }}
        onKeyDown={handleKeyDownEvent}
        onMouseMove={setMousePosition}
        onContextMenu={(event: MouseEvent) => {
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
