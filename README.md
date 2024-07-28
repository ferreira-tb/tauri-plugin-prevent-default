# tauri-plugin-prevent-default

Disable default browser shortcuts in your Tauri app, e.g. `F3` or `Ctrl+J`.

## Install

Install the plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-prevent-default = 0.2
```

Enable the required permissions:

`src-tauri/capabilities/prevent-default.json`

```json
{
  "identifier": "prevent-default",
  "windows": ["*"],
  "permissions": ["prevent-default:default"]
}
```

**OPTIONAL:** Install the JavaScript package with your preferred package manager:

```sh
pnpm add tauri-plugin-prevent-default
```

> Installing the JavaScript package is completely optional. Its only purpose is to provide a way to set simple listeners using JavaScript. `event:allow-listen` should be added to the capabilities file if you decide to use it.

## Usage

Register the plugin with Tauri:

`src-tauri/src/main.rs`

```rust
tauri::Builder::default()
  .plugin(tauri_plugin_prevent_default::init())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

You can also use flags to determine which shortcuts the plugin should disable. By default, it will disable all of them.

```rust
use tauri_plugin_prevent_default::Flags;

let prevent = tauri_plugin_prevent_default::Builder::new()
  .with_flags(Flags::CONTEXT_MENU | Flags::PRINT | Flags::DOWNLOADS)
  .build();

tauri::Builder::default()
  .plugin(prevent)
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

Disable all but a few:

```rust
use tauri_plugin_prevent_default::Flags;

// This will disable all shortcuts, except `FIND` and `RELOAD`.
tauri_plugin_prevent_default::Builder::new()
  .with_flags(Flags::all().difference(Flags::FIND | Flags::RELOAD))
  .build()
```

Disable only keyboard shortcuts:

```rust
use tauri_plugin_prevent_default::Flags;

tauri_plugin_prevent_default::Builder::new()
  .with_flags(Flags::keyboard())
  .build()
```

Disable custom shortcuts:

```rust
use tauri_plugin_prevent_default::KeyboardShortcut;
use tauri_plugin_prevent_default::ModifierKey::{CtrlKey, ShiftKey};

tauri_plugin_prevent_default::Builder::new()
  .shortcut(KeyboardShortcut::new("F12"))
  .shortcut(KeyboardShortcut::with_modifiers("E", &[CtrlKey, ShiftKey]))
  .build();
```

Set a custom event listener:

```rust
use tauri_plugin_prevent_default::Flags;

tauri_plugin_prevent_default::Builder::new()
  .on_flag_event(Flags::CONTEXT_MENU, |window| {
     println!("context menu triggered on window {}", window.label());
  })
 .build();
```

Keep certain shortcuts enabled only when in dev mode:

```rust
fn main() {
  tauri::Builder::default()
    .plugin(prevent_default())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
  use tauri_plugin_prevent_default::Flags;

  tauri_plugin_prevent_default::Builder::new()
    .with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD))
    .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
  tauri_plugin_prevent_default::Builder::new().build()
}
```

## Note

The plugin should work fine on Windows, but there are still tests to be done on MacOS and Linux. If you encounter any problems on these platforms, please [open an issue](https://github.com/ferreira-tb/tauri-plugin-prevent-default/issues).

## Contributing

If there is any other shortcuts that I can include in the plugin, please let me know!
