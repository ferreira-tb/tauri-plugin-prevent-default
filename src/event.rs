use crate::error::Result;
use serde::Serialize;
use std::fmt;
use strum::EnumIs;
use tauri::{AppHandle, Emitter, EventTarget, Manager, Runtime, Window};

/// Define how the plugin should emit events.
#[derive(Clone, Debug, Default, EnumIs)]
pub enum EmitPolicy {
  /// Emit the event to all windows.
  All,
  /// Emit the event to the window with the specified label.
  Custom(String),
  /// Emit the event to the focused window, if any.
  Focused,
  /// Emit the event only to the window that triggered the shortcut.
  Origin,

  #[default]
  /// Do not emit events.
  None,
}

#[derive(Debug, Default)]
pub(crate) struct EventEmitter(pub(crate) EmitPolicy);

impl EventEmitter {
  pub(crate) fn emit<R: Runtime, S>(
    &self,
    window: &Window<R>,
    event: &str,
    payload: S,
  ) -> Result<()>
  where
    S: Serialize + Clone + fmt::Debug,
  {
    let app = window.app_handle();
    match &self.0 {
      EmitPolicy::All => {
        emit_all(app, event, payload)?;
      }
      EmitPolicy::Custom(target) => {
        emit_to(app, event, target, payload)?;
      }
      EmitPolicy::Focused => {
        // We should use `Manager::get_focused_window` when it becomes stable.
        // See: https://docs.rs/tauri/2.0.0-beta/tauri/trait.Manager.html#method.get_focused_window
        let windows = app.webview_windows();
        let window = windows
          .values()
          .find(|w| w.is_focused().is_ok_and(|f| f));

        if let Some(window) = window {
          emit_to(app, event, window.label(), payload)?;
        }
      }
      EmitPolicy::None => {}
      EmitPolicy::Origin => {
        let target = window.label();
        emit_to(app, event, target, payload)?;
      }
    }

    Ok(())
  }
}

fn emit_all<R: Runtime, S>(app: &AppHandle<R>, event: &str, payload: S) -> Result<()>
where
  S: Serialize + Clone + fmt::Debug,
{
  app.emit_filter(event, payload, |target| {
    matches!(target, EventTarget::Window { .. })
  })?;

  Ok(())
}

fn emit_to<R: Runtime, S>(app: &AppHandle<R>, event: &str, target: &str, payload: S) -> Result<()>
where
  S: Serialize + Clone + fmt::Debug,
{
  let target = EventTarget::window(target);
  app
    .emit_to(target, event, payload)
    .map_err(Into::into)
}
