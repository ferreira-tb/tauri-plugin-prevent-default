use super::ModifierKey;
use crate::display;
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

  /// Initialize a new keyboard shortcut builder with the specified key.
  pub fn builder(key: impl AsRef<str>) -> KeyboardShortcutBuilder<R> {
    KeyboardShortcutBuilder::new(key)
  }

  /// Create a new keyboard shortcut with the specified key and modifiers.
  pub fn with_modifiers(key: impl AsRef<str>, modifiers: &[ModifierKey]) -> Self {
    Self::builder(key).modifiers(modifiers).build()
  }

  /// Create a new keyboard shortcut with the specified key and the `AltKey` modifier.
  ///
  /// # Example
  /// ```
  /// use tauri_plugin_prevent_default::KeyboardShortcut;
  /// use tauri_plugin_prevent_default::ModifierKey::AltKey;
  ///
  /// // Both of these are equivalent.
  /// tauri_plugin_prevent_default::Builder::new()
  ///   .shortcut(KeyboardShortcut::with_alt("A"))
  ///   .shortcut(KeyboardShortcut::with_modifiers("A", &[AltKey]))
  ///   .build();
  /// ```
  pub fn with_alt(key: impl AsRef<str>) -> Self {
    Self::builder(key).alt_key().build()
  }

  /// Create a new keyboard shortcut with the specified key and the `CtrlKey` modifier.
  pub fn with_ctrl(key: impl AsRef<str>) -> Self {
    Self::builder(key).ctrl_key().build()
  }

  /// Create a new keyboard shortcut with the specified key and the `CtrlKey` and `MetaKey` modifiers.
  pub fn with_ctrl_meta(key: impl AsRef<str>) -> Self {
    Self::builder(key)
      .modifiers(&[ModifierKey::CtrlKey, ModifierKey::MetaKey])
      .build()
  }

  /// Create a new keyboard shortcut with the specified key and the `CtrlKey` and `ShiftKey` modifiers.
  pub fn with_ctrl_shift(key: impl AsRef<str>) -> Self {
    Self::builder(key)
      .modifiers(&[ModifierKey::CtrlKey, ModifierKey::ShiftKey])
      .build()
  }

  /// Create a new keyboard shortcut with the specified key and the `MetaKey` modifier.
  pub fn with_meta(key: impl AsRef<str>) -> Self {
    Self::builder(key).meta_key().build()
  }

  /// Create a new keyboard shortcut with the specified key and the `ShiftKey` modifier.
  pub fn with_shift(key: impl AsRef<str>) -> Self {
    Self::builder(key).shift_key().build()
  }

  /// Create a new keyboard shortcut with the specified key and the `ShiftKey` and `AltKey` modifiers.
  pub fn with_shift_alt(key: impl AsRef<str>) -> Self {
    Self::builder(key)
      .modifiers(&[ModifierKey::ShiftKey, ModifierKey::AltKey])
      .build()
  }

  /// Create a new keyboard shortcut with the specified key and the `ShiftKey` and `MetaKey` modifiers.
  pub fn with_shift_meta(key: impl AsRef<str>) -> Self {
    Self::builder(key)
      .modifiers(&[ModifierKey::ShiftKey, ModifierKey::MetaKey])
      .build()
  }

  /// Create a new keyboard shortcut with the specified key and a listener.
  pub fn with_listener<F>(key: impl AsRef<str>, listener: F) -> Self
  where
    F: Fn(&Window<R>) + Send + Sync + 'static,
  {
    Self::builder(key).on(listener).build()
  }

  /// The key of the shortcut.
  pub fn key(&self) -> &str {
    &self.key
  }

  /// The modifiers of the shortcut.
  pub fn modifiers(&self) -> &[ModifierKey] {
    self.modifiers.as_slice()
  }
}

impl<R: Runtime> fmt::Display for KeyboardShortcut<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display::keyboard(&self.key, &self.modifiers))
  }
}

#[derive(Debug)]
pub struct KeyboardShortcutBuilder<R: Runtime> {
  key: String,
  modifiers: Vec<ModifierKey>,
  listeners: Vec<EventListener<R>>,
}

impl<R: Runtime> KeyboardShortcutBuilder<R> {
  /// Create a new keyboard shortcut builder with the specified key.
  pub fn new(key: impl AsRef<str>) -> Self {
    Self {
      key: key.as_ref().to_owned(),
      modifiers: Vec::new(),
      listeners: Vec::new(),
    }
  }

  /// Add a modifier to the shortcut.
  #[must_use]
  pub fn modifier(mut self, modifier: ModifierKey) -> Self {
    self.modifiers.push(modifier);
    self
  }

  /// Add multiple modifiers to the shortcut.
  #[must_use]
  pub fn modifiers(mut self, modifiers: &[ModifierKey]) -> Self {
    self.modifiers.extend_from_slice(modifiers);
    self
  }

  /// Add the `AltKey` modifier to the shortcut.
  #[must_use]
  pub fn alt_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::AltKey);
    self
  }

  /// Add the `CtrlKey` modifier to the shortcut.
  #[must_use]
  pub fn ctrl_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::CtrlKey);
    self
  }

  /// Add the `MetaKey` modifier to the shortcut.
  #[must_use]
  pub fn meta_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::MetaKey);
    self
  }

  /// Add the `ShiftKey` modifier to the shortcut.
  #[must_use]
  pub fn shift_key(mut self) -> Self {
    self.modifiers.push(ModifierKey::ShiftKey);
    self
  }

  /// Set a listener for the shortcut.
  #[must_use]
  pub fn on<F>(mut self, listener: F) -> Self
  where
    F: Fn(&Window<R>) + Send + Sync + 'static,
  {
    let listener = EventListener::new(listener);
    self.listeners.push(listener);
    self
  }

  /// Build the keyboard shortcut.
  pub fn build(self) -> KeyboardShortcut<R> {
    KeyboardShortcut {
      key: self.key,
      modifiers: self.modifiers,
      listeners: self.listeners,
    }
  }
}
