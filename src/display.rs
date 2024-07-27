use crate::shortcut::{ModifierKey, PointerEvent};
use itertools::Itertools;

pub(crate) fn keyboard_to_string(key: &str, modifiers: &[ModifierKey]) -> String {
  let mut string = String::from("keyboard:");
  let capacity = key.len() + modifiers.len() * 5;
  string.reserve(capacity);

  for modifier in modifiers.iter().unique().sorted() {
    match modifier {
      ModifierKey::CtrlKey => string.push_str("ctrl+"),
      ModifierKey::ShiftKey => string.push_str("shift+"),
      ModifierKey::AltKey => string.push_str("alt+"),
    }
  }

  string.push_str(&key.to_lowercase());
  string
}

pub(crate) fn pointer_to_string(event: PointerEvent) -> String {
  format!("pointer:{event}")
}

#[cfg(test)]
mod test {
  use crate::shortcut::ModifierKey::{AltKey, CtrlKey, ShiftKey};
  use crate::shortcut::PointerEvent;

  #[test]
  fn keyboard_to_string() {
    use super::keyboard_to_string as k;

    assert_eq!(k("A", &[]), "keyboard:a");
    assert_eq!(k("A", &[CtrlKey]), "keyboard:ctrl+a");
    assert_eq!(k("A", &[ShiftKey, CtrlKey]), "keyboard:ctrl+shift+a");
    assert_eq!(
      k("A", &[ShiftKey, AltKey, CtrlKey]),
      "keyboard:ctrl+shift+alt+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, AltKey, CtrlKey, CtrlKey]),
      "keyboard:ctrl+shift+alt+a"
    );
    assert_eq!(
      k("A", &[ShiftKey, AltKey, CtrlKey, CtrlKey, ShiftKey]),
      "keyboard:ctrl+shift+alt+a"
    );
  }

  #[test]
  fn pointer_to_string() {
    use super::pointer_to_string as p;

    assert_eq!(p(PointerEvent::ContextMenu), "pointer:contextmenu");
  }
}
