//! # tauri-plugin-prevent-default
//!
//! Disable default browser shortcuts in your Tauri app, e.g. `F3` or `Ctrl+J`.
//!
//! ## Install
//!
//! Install the plugin by adding the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! tauri-plugin-prevent-default = 0.5
//! ```
//!
//! If using custom listeners, you must also enable the required permissions:
//!
//! `src-tauri/capabilities/prevent-default.json`
//!
//! ```json
//! {
//!   "identifier": "prevent-default",
//!   "windows": ["*"],
//!   "permissions": ["prevent-default:default"]
//! }
//! ```
//!
//! ## Usage
//!
//! Register the plugin with Tauri:
//!
//! `src-tauri/src/main.rs`
//!
//! ```rust
//! tauri::Builder::default()
//!   .plugin(tauri_plugin_prevent_default::init())
//!   .run(tauri::generate_context!())
//!   .expect("error while running tauri application");
//! ```
//!
//! You can also use flags to determine which shortcuts the plugin should disable. By default, it will disable all of them.
//!
//! ```rust
//! use tauri_plugin_prevent_default::Flags;
//!
//! let prevent = tauri_plugin_prevent_default::Builder::new()
//!   .with_flags(Flags::CONTEXT_MENU | Flags::PRINT | Flags::DOWNLOADS)
//!   .build();
//!
//! tauri::Builder::default()
//!   .plugin(prevent)
//!   .run(tauri::generate_context!())
//!   .expect("error while running tauri application");
//! ```
//!
//! Disable all but a few:
//!
//! ```rust
//! use tauri_plugin_prevent_default::Flags;
//!
//! // This will disable all shortcuts, except `FIND` and `RELOAD`.
//! tauri_plugin_prevent_default::Builder::new()
//!   .with_flags(Flags::all().difference(Flags::FIND | Flags::RELOAD))
//!   .build()
//! ```
//!
//! Disable only keyboard shortcuts:
//!
//! ```rust
//! use tauri_plugin_prevent_default::Flags;
//!
//! tauri_plugin_prevent_default::Builder::new()
//!   .with_flags(Flags::keyboard())
//!   .build()
//! ```
//!
//! Disable custom shortcuts:
//!
//! ```rust
//! use tauri_plugin_prevent_default::KeyboardShortcut;
//! use tauri_plugin_prevent_default::ModifierKey::{CtrlKey, ShiftKey};
//!
//! tauri_plugin_prevent_default::Builder::new()
//!   .shortcut(KeyboardShortcut::new("F12"))
//!   .shortcut(KeyboardShortcut::with_modifiers("E", &[CtrlKey, ShiftKey]))
//!   .shortcut(KeyboardShortcut::with_shift_alt("I"))
//!   .build();
//! ```
//!
//! Keep certain shortcuts enabled only when in dev mode:
//!
//! ```rust
//! fn main() {
//!   tauri::Builder::default()
//!     .plugin(prevent_default())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! }
//!
//! #[cfg(debug_assertions)]
//! fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
//!   use tauri_plugin_prevent_default::Flags;
//!
//!   tauri_plugin_prevent_default::Builder::new()
//!     .with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD))
//!     .build()
//! }
//!
//! #[cfg(not(debug_assertions))]
//! fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
//!   tauri_plugin_prevent_default::init()
//! }
//! ```
//!
//! ## Note
//!
//! The plugin should work fine on Windows, but there are still tests to be done on MacOS and Linux. If you encounter any problems on these platforms, please [open an issue](https://github.com/ferreira-tb/tauri-plugin-prevent-default/issues).
//!

#![forbid(unsafe_code)]
#![cfg(not(any(target_os = "android", target_os = "ios")))]

mod command;
mod display;
mod error;
mod listener;
mod shortcut;
mod state;

use bitflags::bitflags;
pub use error::Error;
pub use shortcut::{
  KeyboardShortcut, KeyboardShortcutBuilder, ModifierKey, PointerEvent, PointerShortcut,
  PointerShortcutBuilder, Shortcut, ShortcutKind,
};
use state::PluginState;
use tauri::plugin::{Builder as PluginBuilder, TauriPlugin};
use tauri::{Manager, Runtime};

#[cfg(feature = "ahash")]
use ahash::{HashSet, HashSetExt};
#[cfg(not(feature = "ahash"))]
use std::collections::HashSet;

bitflags! {
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct Flags: u32 {
      /// Find (`Ctrl+F`, `Ctrl+G`, `Ctrl+Shift+G`, `F3`)
      const FIND            = 1 << 0;
      /// Caret browsing (`F7`)
      const CARET_BROWSING  = 1 << 1;
      /// Developer tools (`Ctrl+Shift+I`)
      const DEV_TOOLS       = 1 << 2;
      /// Downloads (`Ctrl+J`)
      const DOWNLOADS       = 1 << 3;
      /// Focus move (`Shift+Tab`)
      const FOCUS_MOVE      = 1 << 4;
      /// Reload (`F5`, `Ctrl+F5`, `Shift+F5`, `Ctrl+R`, `Ctrl+Shift+R`)
      const RELOAD          = 1 << 5;
      /// Source (`Ctrl+U`)
      const SOURCE          = 1 << 6;
      /// Open (`Ctrl+O`)
      const OPEN            = 1 << 7;
      /// Print document (`Ctrl+P`, `Ctrl+Shift+P`)
      const PRINT           = 1 << 8;
      /// Context menu (mouse right click)
      const CONTEXT_MENU    = 1 << 9;
  }
}

impl Flags {
  /// All keyboard shortcuts.
  pub fn keyboard() -> Self {
    Self::all().difference(Self::pointer())
  }

  /// All pointer shortcuts.
  pub fn pointer() -> Self {
    Self::CONTEXT_MENU
  }
}

impl Default for Flags {
  fn default() -> Self {
    Self::all()
  }
}

pub struct Builder<R: Runtime> {
  flags: Flags,
  shortcuts: Vec<Box<dyn Shortcut<R>>>,
  check_origin: Option<String>,
}

impl<R: Runtime> Default for Builder<R> {
  fn default() -> Self {
    Self {
      flags: Flags::default(),
      shortcuts: Vec::new(),
      check_origin: None,
    }
  }
}

impl<R: Runtime> Builder<R> {
  pub fn new() -> Self {
    Self::default()
  }

  /// Set flags to control which shortcuts the plugin should disable.
  ///
  /// # Examples
  /// ```
  /// use tauri_plugin_prevent_default::Flags;
  ///
  /// tauri_plugin_prevent_default::Builder::new()
  ///   .with_flags(Flags::CONTEXT_MENU | Flags::PRINT | Flags::DOWNLOADS)
  ///   .build();
  /// ```
  #[must_use]
  pub fn with_flags(mut self, flags: Flags) -> Self {
    self.flags = flags;
    self
  }

  /// Disable a custom shortcut.
  ///
  /// # Examples
  /// ```
  /// use tauri_plugin_prevent_default::KeyboardShortcut;
  /// use tauri_plugin_prevent_default::ModifierKey::{CtrlKey, ShiftKey};
  ///
  /// tauri_plugin_prevent_default::Builder::new()
  ///   .shortcut(KeyboardShortcut::new("F12"))
  ///   .shortcut(KeyboardShortcut::with_modifiers("E", &[CtrlKey, ShiftKey]))
  ///   .shortcut(KeyboardShortcut::with_shift_alt("I"))
  ///   .build();
  /// ```
  #[must_use]
  pub fn shortcut<S>(mut self, shortcut: S) -> Self
  where
    S: Shortcut<R> + 'static,
  {
    self.shortcuts.push(Box::new(shortcut));
    self
  }

  /// Check location origin before disabling the shortcuts.
  #[must_use]
  pub fn check_origin(mut self, origin: impl AsRef<str>) -> Self {
    self.check_origin = origin.as_ref().to_owned().into();
    self
  }

  /// Build the plugin.
  pub fn build(mut self) -> TauriPlugin<R> {
    self.add_keyboard_shortcuts();
    self.add_pointer_shortcuts();

    let mut script = String::new();
    let mut state = PluginState::<R>::new();

    for shortcut in &mut self.shortcuts {
      match shortcut.downcast_ref() {
        ShortcutKind::Keyboard(it) => {
          let modifiers = it.modifiers();
          let mut options = String::with_capacity(modifiers.len() * 12);
          for modifier in modifiers {
            options.push_str(&format!("{modifier}:true,"));
          }

          let options = options.trim_end_matches(',');
          script.push_str(&format!("onKey('{}',{{{}}});", it.key(), options));
        }
        ShortcutKind::Pointer(it) => {
          script.push_str(&format!("onPointer('{}');", it.event()));
        }
      }

      let listeners = shortcut.take_listeners();
      if !listeners.is_empty() {
        let shortcut = shortcut.to_string();
        if let Some(it) = state.listeners.get_mut(&shortcut) {
          it.extend(listeners);
        } else {
          #[cfg(feature = "ahash")]
          let set = {
            let mut set = HashSet::new();
            set.extend(listeners);
            set
          };

          #[cfg(not(feature = "ahash"))]
          let set = HashSet::from_iter(listeners);

          state.listeners.insert(shortcut, set);
        }
      }
    }

    let origin = self
      .check_origin
      .map(|it| format!("const ORIGIN='{it}';"))
      .unwrap_or_else(|| "const ORIGIN=null;".to_owned());

    let mut script = include_str!("../scripts/script.js")
      .trim()
      .replace("/*ORIGIN*/", &origin)
      .replace("/*SCRIPT*/", &script);

    if state.listeners.is_empty() {
      script = script.replace("/*EMIT*/", "const EMIT=false;");
    } else {
      script = script.replace("/*EMIT*/", "const EMIT=true;");
    }

    #[cfg(feature = "tracing")]
    tracing::trace!(script);

    let mut builder = PluginBuilder::new("prevent-default");
    if !state.listeners.is_empty() {
      builder = builder
        .invoke_handler(tauri::generate_handler![
          command::keyboard,
          command::pointer
        ])
        .setup(|app, _| {
          app.manage(state);
          Ok(())
        });
    }

    builder.js_init_script(script).build()
  }

  fn add_keyboard_shortcuts(&mut self) {
    use shortcut::ModifierKey::{CtrlKey, ShiftKey};

    macro_rules! on_key {
      ($($arg:literal)+) => {
        $(
          let shortcut = KeyboardShortcut::new($arg);
          self.shortcuts.push(Box::new(shortcut));
        )*
      };
      ($modifiers:expr, $($arg:literal),+) => {
        $(
          let shortcut = KeyboardShortcut::with_modifiers($arg, $modifiers);
          self.shortcuts.push(Box::new(shortcut));
        )*
      };
    }

    if self.flags.contains(Flags::FIND) {
      on_key!("F3");
      on_key!(&[CtrlKey], "f", "g");
      on_key!(&[CtrlKey, ShiftKey], "g");
    }

    if self.flags.contains(Flags::CARET_BROWSING) {
      on_key!("F7");
    }

    if self.flags.contains(Flags::DEV_TOOLS) {
      on_key!(&[CtrlKey, ShiftKey], "i");
    }

    if self.flags.contains(Flags::DOWNLOADS) {
      on_key!(&[CtrlKey], "j");
    }

    if self.flags.contains(Flags::FOCUS_MOVE) {
      on_key!(&[ShiftKey], "Tab");
    }

    if self.flags.contains(Flags::RELOAD) {
      on_key!("F5");
      on_key!(&[CtrlKey], "F5");
      on_key!(&[ShiftKey], "F5");
      on_key!(&[CtrlKey], "r");
      on_key!(&[CtrlKey, ShiftKey], "r");
    }

    if self.flags.contains(Flags::SOURCE) {
      on_key!(&[CtrlKey], "u");
    }

    if self.flags.contains(Flags::OPEN) {
      on_key!(&[CtrlKey], "o");
    }

    if self.flags.contains(Flags::PRINT) {
      on_key!(&[CtrlKey], "p");
      on_key!(&[CtrlKey, ShiftKey], "p");
    }
  }

  fn add_pointer_shortcuts(&mut self) {
    if self.flags.contains(Flags::CONTEXT_MENU) {
      let shortcut = PointerShortcut::new(PointerEvent::ContextMenu);
      self.shortcuts.push(Box::new(shortcut));
    }
  }
}

/// Initialize the plugin with default values.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::default().build()
}

/// Initialize the plugin with given flags.
pub fn with_flags<R: Runtime>(flags: Flags) -> TauriPlugin<R> {
  Builder::new().with_flags(flags).build()
}
