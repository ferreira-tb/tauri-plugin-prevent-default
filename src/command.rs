use crate::shortcut::ModifierKey::{AltKey, CtrlKey, ShiftKey};
use crate::state::PluginState;
use crate::{display, PointerEvent};
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime, Window};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct KeyboardPayload {
  key: String,
  ctrl_key: bool,
  shift_key: bool,
  alt_key: bool,
  origin: Option<String>,
}

#[tauri::command]
pub(crate) async fn keyboard<R: Runtime>(window: Window<R>, payload: KeyboardPayload) {
  #[cfg(feature = "tracing")]
  tracing::trace!(kind = "keyboard", window = window.label(), ?payload);

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

  let shortcut = display::keyboard(&payload.key, &modifiers);
  let state = window.state::<PluginState<R>>();
  state.call_listeners(&shortcut, &window);
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PointerPayload {
  name: String,
  origin: Option<String>,
}

#[tauri::command]
pub(crate) async fn pointer<R: Runtime>(window: Window<R>, payload: PointerPayload) {
  #[cfg(feature = "tracing")]
  tracing::trace!(kind = "pointer", window = window.label(), ?payload);

  let name = payload.name.as_str();
  if let Ok(event) = PointerEvent::try_from(name) {
    let state = window.state::<PluginState<R>>();
    state.call_listeners(&display::pointer(event), &window);
  }
}
