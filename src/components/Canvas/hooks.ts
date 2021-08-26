import { KeyboardEvent, useRef, MutableRefObject } from 'react';

export function useKeyPress(
  callbackOnEachKeyDown: (keyPressSetReference: MutableRefObject<Set<string>>) => void,
): [({ key }: KeyboardEvent<HTMLCanvasElement>) => void, ({ key }: KeyboardEvent<HTMLCanvasElement>) => void] {
  // State for keeping track of whether key is pressed
  const keyPressSetReference = useRef(new Set<string>([]));

  function downHandler(event: KeyboardEvent<HTMLCanvasElement>): void {
    keyPressSetReference.current.add(event.key);
    console.log(`keyPressSetReference.current`, keyPressSetReference.current);
    callbackOnEachKeyDown(keyPressSetReference);
  }
  const upHandler = (event: KeyboardEvent<HTMLCanvasElement>): void => {
    keyPressSetReference.current.delete(event.key);
  };
  return [downHandler, upHandler];
}
