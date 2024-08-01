use crate::display;
use crate::listener::EventListener;
use std::fmt;
use strum::{Display as EnumDisplay, EnumIs, EnumString};
use tauri::{Runtime, Window};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumDisplay, EnumIs, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum PointerEvent {
  ContextMenu,
}

#[derive(Debug)]
pub struct PointerShortcut<R: Runtime> {
  event: PointerEvent,
  pub(super) listeners: Vec<EventListener<R>>,
}

impl<R: Runtime> PointerShortcut<R> {
  pub fn new(event: PointerEvent) -> Self {
    Self { event, listeners: Vec::new() }
  }

  pub fn builder(event: PointerEvent) -> PointerShortcutBuilder<R> {
    PointerShortcutBuilder::new(event)
  }

  pub fn with_listener<F>(event: PointerEvent, listener: F) -> Self
  where
    F: Fn(&Window<R>) + Send + Sync + 'static,
  {
    Self::builder(event).on(listener).build()
  }

  pub fn event(&self) -> PointerEvent {
    self.event
  }
}

impl<R: Runtime> fmt::Display for PointerShortcut<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", display::pointer(self.event))
  }
}

#[derive(Debug)]
pub struct PointerShortcutBuilder<R: Runtime> {
  event: PointerEvent,
  listeners: Vec<EventListener<R>>,
}

impl<R: Runtime> PointerShortcutBuilder<R> {
  pub fn new(event: PointerEvent) -> Self {
    Self { event, listeners: Vec::new() }
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

  pub fn build(self) -> PointerShortcut<R> {
    PointerShortcut {
      event: self.event,
      listeners: self.listeners,
    }
  }
}
