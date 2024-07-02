//! Disable default browser shortcuts in your Tauri app, e.g. `F3` or `Ctrl+J`.

#![cfg(not(any(target_os = "android", target_os = "ios")))]

use bitflags::bitflags;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;

bitflags! {
  #[derive(Clone, Copy, Debug)]
  pub struct Flags: u32 {
      /// Find (`Ctrl+F`, `Ctrl+G`, `Ctrl+Shift+G`, `F3`)
      const FIND          = 1 << 0;
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
      const SOURCE          = 1 << 7;
      /// Open (`Ctrl+O`)
      const OPEN            = 1 << 8;
      /// Print document (`Ctrl+P`, `Ctrl+Shift+P`)
      const PRINT           = 1 << 9;
      /// Context menu (mouse right click)
      const CONTEXT_MENU    = 1 << 10;
  }
}

impl Default for Flags {
  fn default() -> Self {
    Self::all()
  }
}

const SCRIPT: &str = r#"
  (() => {
    function createPredicate(key) {
      if (Array.isArray(key)) return (e) => key.includes(e.key);
      return (e) => e.key === key;
    }

    function onKey(key, options = {}) {
      const predicate = createPredicate(key);
      const { altKey = false, ctrlKey = false, shiftKey = false } = options;

      globalThis.addEventListener('keydown', (e) => {
        if (e.altKey !== altKey || e.ctrlKey !== ctrlKey || e.shiftKey !== shiftKey) {
          return;
        }

        if (predicate(e)) {
          e.preventDefault();
        }
      });
    }

    //REPLACE_ME
  })();
"#;

#[derive(Default)]
pub struct Builder {
  flags: Flags,
}

impl Builder {
  pub fn new() -> Self {
    Self::default()
  }

  /// Set flags to control which shortcuts the plugin should disable.
  pub fn with_flags(mut self, flags: Flags) -> Self {
    self.flags = flags;
    self
  }

  pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
    let mut js = String::new();

    if self.flags.contains(Flags::FIND) {
      js.push_str("onKey('F3');");
      js.push_str("onKey(['f', 'F', 'g', 'G'], { ctrlKey: true });");
      js.push_str("onKey(['g', 'G'], { ctrlKey: true, shiftKey: true });");
    }

    if self.flags.contains(Flags::CARET_BROWSING) {
      js.push_str("onKey('F7');");
    }

    if self.flags.contains(Flags::DEV_TOOLS) {
      js.push_str("onKey(['i', 'I'], { ctrlKey: true, shiftKey: true });");
    }

    if self.flags.contains(Flags::DOWNLOADS) {
      js.push_str("onKey(['j', 'J'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::FOCUS_MOVE) {
      js.push_str("onKey('Tab', { shiftKey: true });");
    }

    if self.flags.contains(Flags::RELOAD) {
      js.push_str("onKey('F5');");
      js.push_str("onKey('F5', { ctrlKey: true });");
      js.push_str("onKey('F5', { shiftKey: true });");
      js.push_str("onKey(['r', 'R'], { ctrlKey: true });");
      js.push_str("onKey(['r', 'R'], { ctrlKey: true, shiftKey: true });");
    }

    if self.flags.contains(Flags::SOURCE) {
      js.push_str("onKey(['u', 'U'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::OPEN) {
      js.push_str("onKey(['o', 'O'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::PRINT) {
      js.push_str("onKey(['p', 'P'], { ctrlKey: true });");
      js.push_str("onKey(['p', 'P'], { ctrlKey: true, shiftKey: true });");
    }

    if self.flags.contains(Flags::CONTEXT_MENU) {
      js.push_str(
        "globalThis.addEventListener('contextmenu', (e) => {
          e.preventDefault();
        });",
      );
    }

    let script = SCRIPT.replace("//REPLACE_ME", &js);

    tauri::plugin::Builder::new("prevent-default")
      .js_init_script(script)
      .build()
  }
}
