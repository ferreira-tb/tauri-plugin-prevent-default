mod keyboard;
mod pointer;

use std::fmt;
use strum::{Display, EnumIs};

pub use keyboard::{KeyboardShortcut, KeyboardShortcutBuilder};
pub use pointer::{PointerEvent, PointerShortcut, PointerShortcutBuilder};

pub trait Shortcut: fmt::Display {
  fn kind(&self) -> ShortcutKind;
}

impl Shortcut for KeyboardShortcut {
  fn kind(&self) -> ShortcutKind {
    ShortcutKind::Keyboard(self)
  }
}

impl Shortcut for PointerShortcut {
  fn kind(&self) -> ShortcutKind {
    ShortcutKind::Pointer(self)
  }
}

#[derive(Debug)]
pub enum ShortcutKind<'a> {
  Keyboard(&'a KeyboardShortcut),
  Pointer(&'a PointerShortcut),
}

impl ShortcutKind<'_> {
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
  MetaKey,
  ShiftKey,
}

impl ModifierKey {
  fn precedence(self) -> u8 {
    match self {
      ModifierKey::CtrlKey => 0,
      ModifierKey::ShiftKey => 1,
      ModifierKey::AltKey => 2,
      ModifierKey::MetaKey => 3,
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
  use super::ModifierKey::{AltKey, CtrlKey, MetaKey, ShiftKey};
  use super::*;

  #[test]
  fn shortcut_kind() {
    // Keyboard
    let keyboard = KeyboardShortcut::new("F12");
    let keyboard = Box::new(keyboard) as Box<dyn Shortcut>;
    assert!(keyboard.kind().is_keyboard());

    // Pointer
    let pointer = PointerShortcut::new(PointerEvent::ContextMenu);
    let pointer = Box::new(pointer) as Box<dyn Shortcut>;
    assert!(pointer.kind().is_pointer());
  }

  #[test]
  fn modifier_key_order() {
    assert!(CtrlKey < ShiftKey);
    assert!(ShiftKey < AltKey);
    assert!(CtrlKey < AltKey);

    let mut modifiers = vec![AltKey, MetaKey, CtrlKey, ShiftKey];
    modifiers.sort();

    assert_eq!(modifiers, vec![CtrlKey, ShiftKey, AltKey, MetaKey]);
  }
}
