use crate::display;
use std::fmt;
use strum::{Display as EnumDisplay, EnumIs, EnumString};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum PointerEvent {
  ContextMenu,
}

#[derive(Debug)]
pub struct PointerShortcut {
  event: PointerEvent,
}

impl PointerShortcut {
  pub fn new(event: PointerEvent) -> Self {
    Self { event }
  }

  /// Initialize a new pointer shortcut builder with the specified event.
  pub fn builder(event: PointerEvent) -> PointerShortcutBuilder {
    PointerShortcutBuilder::new(event)
  }

  /// The event of the shortcut.
  pub fn event(&self) -> PointerEvent {
    self.event
  }
}

impl fmt::Display for PointerShortcut {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display::pointer(self.event))
  }
}

#[derive(Debug)]
pub struct PointerShortcutBuilder {
  event: PointerEvent,
}

impl PointerShortcutBuilder {
  pub fn new(event: PointerEvent) -> Self {
    Self { event }
  }

  /// Build the pointer shortcut.
  pub fn build(self) -> PointerShortcut {
    PointerShortcut { event: self.event }
  }
}
