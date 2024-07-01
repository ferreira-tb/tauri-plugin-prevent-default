use bitflags::bitflags;
use tauri::plugin::TauriPlugin;
use tauri::Runtime;

bitflags! {
  #[derive(Clone, Copy, Debug)]
  pub struct Flags: u32 {
      const SEARCH          = 1 << 0;
      const CARET_BROWSING  = 1 << 1;
      const DEV_TOOLS       = 1 << 2;
      const DOWNLOADS       = 1 << 3;
      const FOCUS_MOVE      = 1 << 4;
      const RELOAD          = 1 << 5;
      const SOURCE          = 1 << 7;
      const OPEN            = 1 << 8;
      const PRINT           = 1 << 9;
      const CONTEXT_MENU    = 1 << 10;
  }
}

impl Default for Flags {
  fn default() -> Self {
    Self::all()
  }
}

const SCRIPT: &str = r#"
  if (window.location.origin === 'https://tauri.app') {
    (() => {
      function createPredicate(key) {
        if (Array.isArray(key)) return (e) => key.includes(e.key);
        return (e) => e.key === key;
      }

      function onKey(key, options) {
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
  }
"#;

#[derive(Default)]
pub struct Builder {
  flags: Flags,
}

impl Builder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_state_flags(mut self, flags: Flags) -> Self {
    self.flags = flags;
    self
  }

  pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
    let mut js = String::new();

    if self.flags.contains(Flags::SEARCH) {
      js.push_str("onKey('F3');");
      js.push_str("onKey(['f', 'F', 'g', 'G'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::CARET_BROWSING) {
      js.push_str("onKey('F7');");
    }

    if self.flags.contains(Flags::DEV_TOOLS) {
      js.push_str("onKey('i', { ctrlKey: true, shiftKey: true });");
    }

    if self.flags.contains(Flags::DOWNLOADS) {
      js.push_str("onKey(['j', 'J'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::FOCUS_MOVE) {
      js.push_str("onKeyDown('Tab', { shiftKey: true });");
    }

    if self.flags.contains(Flags::RELOAD) {
      js.push_str("onKeyDown(['r', 'R'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::SOURCE) {
      js.push_str("onKeyDown(['u', 'U'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::OPEN) {
      js.push_str("onKeyDown(['o', 'O'], { ctrlKey: true });");
    }

    if self.flags.contains(Flags::PRINT) {
      js.push_str("onKeyDown(['p', 'P'], { ctrlKey: true });");
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
