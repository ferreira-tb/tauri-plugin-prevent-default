use super::ModifierKey;
use crate::display::keyboard_to_string;
use crate::listener::EventListener;
use std::fmt;
use tauri::{Runtime, Window};

#[derive(Debug)]
pub struct KeyboardShortcut<R: Runtime> {
  key: String,
  modifiers: Vec<ModifierKey>,
  pub(super) listeners: Vec<EventListener<R>>,
}

impl<R: Runtime> KeyboardShortcut<R> {
  pub fn new(key: impl AsRef<str>) -> Self {
    Self {
      key: key.as_ref().to_owned(),
      modifiers: Vec::new(),
      listeners: Vec::new(),
    }
  }

  pub fn with_modifiers(key: impl AsRef<str>, modifiers: &[ModifierKey]) -> Self {
    Self::builder(key).modifiers(modifiers).build()
  }

  pub fn builder(key: impl AsRef<str>) -> KeyboardShortcutBuilder<R> {
    KeyboardShortcutBuilder::new(key)
  }

  pub fn key(&self) -> &str {
    &self.key
  }

  pub fn modifiers(&self) -> &[ModifierKey] {
    self.modifiers.as_slice()
  }
}

impl<R: Runtime> fmt::Display for KeyboardShortcut<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", keyboard_to_string(&self.key, &self.modifiers))
  }
}

#[derive(Debug)]
pub struct KeyboardShortcutBuilder<R: Runtime> {
  key: String,
  modifiers: Vec<ModifierKey>,
  listeners: Vec<EventListener<R>>,
}

impl<R: Runtime> KeyboardShortcutBuilder<R> {
  pub fn new(key: impl AsRef<str>) -> Self {
    Self {
      key: key.as_ref().to_owned(),
      modifiers: Vec::new(),
      listeners: Vec::new(),
    }
  }

  pub fn modifier(mut self, modifier: ModifierKey) -> Self {
    self.modifiers.push(modifier);
    self
  }

  pub fn modifiers(mut self, modifiers: &[ModifierKey]) -> Self {
    self.modifiers.extend_from_slice(modifiers);
    self
  }

  pub fn alt_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::AltKey);
    self
  }

  pub fn ctrl_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::CtrlKey);
    self
  }

  pub fn shift_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::ShiftKey);
    self
  }

  /// Set a listener for the shortcut.
  pub fn on<F>(mut self, listener: F) -> Self
  where
    F: Fn(&Window<R>) + Send + Sync + 'static,
  {
    let listener = EventListener::new(listener);
    self.listeners.push(listener);
    self
  }

  pub fn build(self) -> KeyboardShortcut<R> {
    KeyboardShortcut {
      key: self.key,
      modifiers: self.modifiers,
      listeners: self.listeners,
    }
  }
}
