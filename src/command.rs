use crate::display::{keyboard_to_string, pointer_to_string};
use crate::error::Result;
use crate::shortcut::ModifierKey::{AltKey, CtrlKey, ShiftKey};
use crate::state::PluginState;
use crate::PointerEvent;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime, Window};

const KEYBOARD_EVENT: &str = "prevent-default://keyboard";
const POINTER_EVENT: &str = "prevent-default://pointer";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KeyboardPayload {
  key: String,
  ctrl_key: bool,
  shift_key: bool,
  alt_key: bool,

  #[serde(default)]
  origin: Option<String>,
}

#[tauri::command]
pub(crate) async fn keyboard<R: Runtime>(
  window: Window<R>,
  mut payload: KeyboardPayload,
) -> Result<()> {
  #[cfg(feature = "tracing")]
  tracing::debug!(kind = "keyboard", window = window.label(), ?payload);

  let mut modifiers = Vec::new();
  if payload.alt_key {
    modifiers.push(AltKey);
  }

  if payload.ctrl_key {
    modifiers.push(CtrlKey);
  }

  if payload.shift_key {
    modifiers.push(ShiftKey);
  }

  let state = window.state::<PluginState<R>>();
  let shortcut = keyboard_to_string(&payload.key, &modifiers);
  state.call_listeners(&shortcut, &window);

  payload.origin = window.label().to_owned().into();

  state
    .emitter
    .emit(&window, KEYBOARD_EVENT, payload)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PointerPayload {
  event: String,

  #[serde(default)]
  origin: Option<String>,
}

#[tauri::command]
pub(crate) async fn pointer<R: Runtime>(
  window: Window<R>,
  mut payload: PointerPayload,
) -> Result<()> {
  #[cfg(feature = "tracing")]
  tracing::debug!(kind = "pointer", window = window.label(), ?payload);

  let state = window.state::<PluginState<R>>();
  match PointerEvent::try_from(payload.event.as_str()) {
    Ok(event) => {
      state.call_listeners(&pointer_to_string(event), &window);
    }
    #[cfg(feature = "tracing")]
    Err(error) => tracing::error!(%error),
    #[cfg(not(feature = "tracing"))]
    Err(_) => {}
  }

  payload.origin = window.label().to_owned().into();
  state
    .emitter
    .emit(&window, POINTER_EVENT, payload)
}
