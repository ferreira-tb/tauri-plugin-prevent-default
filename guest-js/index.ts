import { listen } from '@tauri-apps/api/event';

interface Payload {
  /** Label of the window from which the event originated. */
  origin: string;
}

interface KeyboardEventPayload extends Payload {
  key: string;
  ctrlKey: boolean;
  shiftKey: boolean;
  altKey: boolean;
}

interface PointerEventPayload extends Payload {
  event: string;
}

/**
 * Listen to keyboard events emitted by the plugin.
 *
 * **IMPORTANT**: `event:allow-listen` must be enabled in your capabilities file.
 */
export function onKeyboardEvent(fn: (event: KeyboardEventPayload) => void) {
  return listen<KeyboardEventPayload>('prevent-default://keyboard', ({ payload }) => {
    fn(payload);
  });
}

/**
 * Listen to pointer events emitted by the plugin.
 *
 * **IMPORTANT**: `event:allow-listen` must be enabled in your capabilities file.
 */
export function onPointerEvent(fn: (event: PointerEventPayload) => void) {
  return listen<PointerEventPayload>('prevent-default://pointer', ({ payload }) => {
    fn(payload);
  });
}

export const on = onKeyboardEvent;
