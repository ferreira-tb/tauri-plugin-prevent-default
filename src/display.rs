use crate::shortcut::{ModifierKey, PointerEvent};
use itertools::Itertools;

pub(crate) fn keyboard(key: &str, modifiers: &[ModifierKey]) -> String {
  let mut string = String::from("keyboard:");
  let capacity = key.len() + modifiers.len() * 5;
  string.reserve(capacity);

  for modifier in modifiers.iter().unique().sorted() {
    match modifier {
      ModifierKey::CtrlKey => string.push_str("ctrl+"),
      ModifierKey::ShiftKey => string.push_str("shift+"),
      ModifierKey::AltKey => string.push_str("alt+"),
      ModifierKey::MetaKey => string.push_str("meta+"),
    }
  }

  string.push_str(&key.to_lowercase());
  string
}

pub(crate) fn pointer(event: PointerEvent) -> String {
  format!("pointer:{event}")
}

#[cfg(test)]
mod test {
  use crate::shortcut::ModifierKey::{AltKey, CtrlKey, MetaKey, ShiftKey};
  use crate::shortcut::PointerEvent;

  #[test]
  #[rustfmt::skip]
  fn display_keyboard() {
    use super::keyboard as k;

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
    use super::pointer as p;

    assert_eq!(p(PointerEvent::ContextMenu), "pointer:contextmenu");
  }
}
