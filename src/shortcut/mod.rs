mod keyboard;
mod pointer;

use crate::listener::EventListener;
use std::{fmt, mem};
use strum::{Display, EnumIs};
use tauri::Runtime;

pub use keyboard::{KeyboardShortcut, KeyboardShortcutBuilder};
pub use pointer::{PointerEvent, PointerShortcut, PointerShortcutBuilder};

pub trait Shortcut<R: Runtime>: fmt::Display {
  #[doc(hidden)]
  fn downcast_ref(&self) -> ShortcutKind<R>;
  #[doc(hidden)]
  fn add_listeners(&mut self, listeners: &[EventListener<R>]);
  #[doc(hidden)]
  fn take_listeners(&mut self) -> Vec<EventListener<R>>;
}

impl<R: Runtime> Shortcut<R> for KeyboardShortcut<R> {
  fn downcast_ref(&self) -> ShortcutKind<R> {
    ShortcutKind::Keyboard(self)
  }

  fn add_listeners(&mut self, listeners: &[EventListener<R>]) {
    self.listeners.extend_from_slice(listeners);
  }

  fn take_listeners(&mut self) -> Vec<EventListener<R>> {
    mem::take(&mut self.listeners)
  }
}

impl<R: Runtime> Shortcut<R> for PointerShortcut<R> {
  fn downcast_ref(&self) -> ShortcutKind<R> {
    ShortcutKind::Pointer(self)
  }

  fn add_listeners(&mut self, listeners: &[EventListener<R>]) {
    self.listeners.extend_from_slice(listeners);
  }

  fn take_listeners(&mut self) -> Vec<EventListener<R>> {
    mem::take(&mut self.listeners)
  }
}

#[derive(Debug)]
pub enum ShortcutKind<'a, R: Runtime> {
  Keyboard(&'a KeyboardShortcut<R>),
  Pointer(&'a PointerShortcut<R>),
}

impl<R: Runtime> ShortcutKind<'_, R> {
  /// Returns `true` if the shortcut is a keyboard shortcut.
  pub fn is_keyboard(&self) -> bool {
    matches!(self, ShortcutKind::Keyboard(_))
  }

  /// Returns `true` if the shortcut is a pointer shortcut.
  pub fn is_pointer(&self) -> bool {
    matches!(self, ShortcutKind::Pointer(_))
  }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash, EnumIs)]
#[strum(serialize_all = "camelCase")]
pub enum ModifierKey {
  AltKey,
  CtrlKey,
  ShiftKey,
}

impl ModifierKey {
  fn precedence(&self) -> u8 {
    match self {
      ModifierKey::CtrlKey => 0,
      ModifierKey::ShiftKey => 1,
      ModifierKey::AltKey => 2,
    }
  }
}

impl PartialOrd for ModifierKey {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ModifierKey {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.precedence().cmp(&other.precedence())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use tauri::Wry;

  #[test]
  fn take_listeners() {
    // Keyboard
    let mut keyboard = KeyboardShortcut::<Wry>::new("F12");
    let listener = EventListener::new(|_| {});
    keyboard.listeners.push(listener.clone());

    let listeners = keyboard.take_listeners();
    assert_eq!(listeners, vec![listener]);
    assert!(keyboard.listeners.is_empty());

    // Pointer
    let mut pointer = PointerShortcut::<Wry>::new(PointerEvent::ContextMenu);
    let listener = EventListener::new(|_| {});
    pointer.listeners.push(listener.clone());

    let listeners = pointer.take_listeners();
    assert_eq!(listeners, vec![listener]);
    assert!(pointer.listeners.is_empty());
  }

  #[test]
  fn shortcut_kind() {
    // Keyboard
    let keyboard = KeyboardShortcut::new("F12");
    let keyboard = Box::new(keyboard) as Box<dyn Shortcut<Wry>>;
    assert!(keyboard.downcast_ref().is_keyboard());

    // Pointer
    let pointer = PointerShortcut::new(PointerEvent::ContextMenu);
    let pointer = Box::new(pointer) as Box<dyn Shortcut<Wry>>;
    assert!(pointer.downcast_ref().is_pointer());
  }

  #[test]
  fn modifier_key_order() {
    assert!(ModifierKey::CtrlKey < ModifierKey::ShiftKey);
    assert!(ModifierKey::ShiftKey < ModifierKey::AltKey);
    assert!(ModifierKey::CtrlKey < ModifierKey::AltKey);

    let mut modifiers = vec![
      ModifierKey::AltKey,
      ModifierKey::CtrlKey,
      ModifierKey::ShiftKey,
    ];

    modifiers.sort();
    assert_eq!(
      modifiers,
      vec![
        ModifierKey::CtrlKey,
        ModifierKey::ShiftKey,
        ModifierKey::AltKey
      ]
    );
  }
}
