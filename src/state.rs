use crate::listener::EventListener;
use std::collections::{HashMap, HashSet};
use tauri::{Runtime, Window};

pub(crate) struct PluginState<R: Runtime> {
  pub(crate) listeners: HashMap<String, HashSet<EventListener<R>>>,
}

impl<R: Runtime> PluginState<R> {
  pub(crate) fn new() -> Self {
    Self { listeners: HashMap::new() }
  }

  pub(crate) fn call_listeners(&self, shortcut: &str, window: &Window<R>) {
    if let Some(listeners) = self.listeners.get(shortcut) {
      listeners
        .iter()
        .for_each(|listener| listener.call(window));
    }
  }
}
