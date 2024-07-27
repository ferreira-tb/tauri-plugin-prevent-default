import { listen } from '@tauri-apps/api/event';

const KEYBOARD_EVENT = 'prevent-default://keyboard';
const POINTER_EVENT = 'prevent-default://pointer';

interface Payload {
  /** Label of the window from which the event originated. */
  origin: string;
}

export interface KeyboardEventPayload extends Payload {
  key: string;
  ctrlKey: boolean;
  shiftKey: boolean;
  altKey: boolean;
}

export interface PointerEventPayload extends Payload {
  event: string;
}

/**
 * Listen to keyboard events emitted by the plugin.
 *
 * **IMPORTANT**: `event:allow-listen` must be enabled in your capabilities file.
 */
export function onKeyboardEvent(fn: (event: KeyboardEventPayload) => void) {
  return listen<KeyboardEventPayload>(KEYBOARD_EVENT, ({ payload }) => {
    fn(payload);
  });
}

/** Alias for {@link onKeyboardEvent}. */
export const on = onKeyboardEvent;

/**
 * Listen to pointer events emitted by the plugin.
 *
 * **IMPORTANT**: `event:allow-listen` must be enabled in your capabilities file.
 */
export function onPointerEvent(fn: (event: PointerEventPayload) => void) {
  return listen<PointerEventPayload>(POINTER_EVENT, ({ payload }) => {
    fn(payload);
  });
}
