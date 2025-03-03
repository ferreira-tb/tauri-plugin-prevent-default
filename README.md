# tauri-plugin-prevent-default

Disable default browser shortcuts in your Tauri app, e.g. `F3` or `Ctrl+J`.

## Install

Install the plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-prevent-default = "1.2"
```

If using custom listeners, you must also enable the required permissions:

`src-tauri/capabilities/prevent-default.json`

```json
{
  "identifier": "prevent-default",
  "windows": ["*"],
  "permissions": ["prevent-default:default"]
}
```

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

- Disable all but a few:

```rust
use tauri_plugin_prevent_default::Flags;

// This will disable all shortcuts, except `FIND` and `RELOAD`.
tauri_plugin_prevent_default::Builder::new()
  .with_flags(Flags::all().difference(Flags::FIND | Flags::RELOAD))
  .build()
```

- Disable only keyboard shortcuts:

```rust
use tauri_plugin_prevent_default::Flags;

tauri_plugin_prevent_default::Builder::new()
  .with_flags(Flags::keyboard())
  .build()
```

- Disable custom shortcuts:

```rust
use tauri_plugin_prevent_default::KeyboardShortcut;
use tauri_plugin_prevent_default::ModifierKey::{CtrlKey, ShiftKey};

tauri_plugin_prevent_default::Builder::new()
  .shortcut(KeyboardShortcut::new("F12"))
  .shortcut(KeyboardShortcut::with_modifiers("E", &[CtrlKey, ShiftKey]))
  .shortcut(KeyboardShortcut::with_shift_alt("I"))
  .build();
```

- Keep certain shortcuts enabled only when in dev mode:

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
  tauri_plugin_prevent_default::init()
}
```

## Platform-specific options

### Windows

The `unstable-windows` feature must be enabled.

```toml
[dependencies]
tauri-plugin-prevent-default = { version = "1.2", features = ["unstable-windows"] }
```

```rust
use tauri_plugin_prevent_default::WindowsOptions;

tauri_plugin_prevent_default::Builder::new()
  .platform(WindowsOptions {
    // Whether general form information should be saved and autofilled.
    general_autofill: true,
    // Whether password information should be autosaved.
    password_autosave: false,
  })
  .build()
```

## Experimental features

[Cargo features](https://doc.rust-lang.org/cargo/reference/features.html) prefixed with `unstable` are experimental and may introduce breaking changes between minor versions.

## Note

The plugin should work fine on Windows, but there are still tests to be done on MacOS and Linux. If you encounter any problems on these platforms, please [open an issue](https://github.com/ferreira-tb/tauri-plugin-prevent-default/issues).

## Supported Tauri Version

This plugin requires Tauri `2.0` or later.
