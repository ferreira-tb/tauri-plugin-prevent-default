use crate::event::EventEmitter;
use crate::listener::EventListener;
use tauri::{Runtime, Window};

#[cfg(feature = "ahash")]
use ahash::{HashMap, HashSet};
#[cfg(not(feature = "ahash"))]
use std::collections::{HashMap, HashSet};

pub(crate) struct PluginState<R: Runtime> {
  pub(crate) emitter: EventEmitter,
  pub(crate) listeners: HashMap<String, HashSet<EventListener<R>>>,
}

impl<R: Runtime> PluginState<R> {
  pub(crate) fn call_listeners(&self, shortcut: &str, window: &Window<R>) {
    if let Some(listeners) = self.listeners.get(shortcut) {
      for listener in listeners {
        #[cfg(feature = "tracing")]
        tracing::debug!(shortcut, window = window.label(), ?listener);

        listener.call(window);
      }
    }
  }
}
