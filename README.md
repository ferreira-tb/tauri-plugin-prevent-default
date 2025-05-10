# tauri-plugin-prevent-default

Disable default browser shortcuts in your Tauri app, e.g. `F3` or `Ctrl+J`.

## Install

Install the plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-prevent-default = "2.0"
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

If you want to keep only [`CONTEXT_MENU`](https://docs.rs/tauri-plugin-prevent-default/latest/tauri_plugin_prevent_default/struct.Flags.html#associatedconstant.CONTEXT_MENU), [`DEV_TOOLS`](https://docs.rs/tauri-plugin-prevent-default/latest/tauri_plugin_prevent_default/struct.Flags.html#associatedconstant.DEV_TOOLS), and [`RELOAD`](https://docs.rs/tauri-plugin-prevent-default/latest/tauri_plugin_prevent_default/struct.Flags.html#associatedconstant.RELOAD) enabled in dev mode, you can build the plugin with [`tauri_plugin_prevent_default::debug`](https://docs.rs/tauri-plugin-prevent-default/latest/tauri_plugin_prevent_default/fn.debug.html) instead.

```rust
tauri::Builder::default()
  .plugin(tauri_plugin_prevent_default::debug())
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
```

## Platform-specific options

Please read our [versioning policy](#versioning-and-experimental-features) before using any of these options.

### Windows

The `unstable-windows` feature must be enabled.

```toml
[dependencies]
tauri-plugin-prevent-default = { version = "2.0", features = ["unstable-windows"] }
```

```rust
use tauri_plugin_prevent_default::PlatformOptions;

tauri_plugin_prevent_default::Builder::new()
  .platform(PlatformOptions {
    // Whether general form information should be saved and autofilled.
    general_autofill: true,
    // Whether password information should be autosaved.
    password_autosave: false,
  })
  .build()
```

## Versioning and Experimental Features

This plugin adheres to [SemVer](https://semver.org/). However, [features](https://doc.rust-lang.org/cargo/reference/features.html) marked as `unstable` are experimental and may introduce breaking changes between minor versions.

## Supported Tauri Version

This plugin requires Tauri `2.0` or later.
