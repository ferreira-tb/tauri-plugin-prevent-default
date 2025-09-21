use crate::shortcut::{ModifierKey, PointerEvent};
use itertools::Itertools;

pub(crate) fn keyboard(key: &str, modifiers: &[ModifierKey]) -> String {
  let mut buf = String::from("keyboard:");
  let mod_len = modifiers.len().saturating_mul(5);
  buf.reserve(key.len().saturating_add(mod_len));

  for modifier in modifiers.iter().unique().sorted() {
    match modifier {
      ModifierKey::CtrlKey => buf.push_str("ctrl+"),
      ModifierKey::ShiftKey => buf.push_str("shift+"),
      ModifierKey::AltKey => buf.push_str("alt+"),
      ModifierKey::MetaKey => buf.push_str("meta+"),
    }
  }

  buf.push_str(&key.to_lowercase());
  buf
}

pub(crate) fn pointer(event: PointerEvent) -> String {
  format!("pointer:{event}")
}

#[cfg(test)]
mod test {
  use super::keyboard as k;
  use super::pointer as p;
  use crate::shortcut::ModifierKey::{AltKey, CtrlKey, MetaKey, ShiftKey};
  use crate::shortcut::PointerEvent;

  #[test]
  #[rustfmt::skip]
  fn display_keyboard() {
    assert_eq!(
      k("A", &[]),
      "keyboard:a"
    );
    assert_eq!(
      k("A", &[CtrlKey]),
      "keyboard:ctrl+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, CtrlKey]),
      "keyboard:ctrl+shift+a"
    );
    assert_eq!(
      k("A", &[MetaKey, ShiftKey]),
      "keyboard:shift+meta+a"
    );
    assert_eq!(
      k("A", &[MetaKey, ShiftKey, CtrlKey]),
      "keyboard:ctrl+shift+meta+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, AltKey, CtrlKey]),
      "keyboard:ctrl+shift+alt+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, AltKey, CtrlKey, CtrlKey]),
      "keyboard:ctrl+shift+alt+a"
    );
    assert_eq!(
      k("A", &[MetaKey, ShiftKey, AltKey, CtrlKey]),
      "keyboard:ctrl+shift+alt+meta+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, AltKey, CtrlKey, CtrlKey, ShiftKey]),
      "keyboard:ctrl+shift+alt+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, MetaKey, AltKey, CtrlKey, MetaKey]),
      "keyboard:ctrl+shift+alt+meta+a"
    );
  }

  #[test]
  fn display_pointer() {
    assert_eq!(p(PointerEvent::ContextMenu), "pointer:contextmenu");
  }
}
