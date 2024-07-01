# tauri-plugin-prevent-default

Disable default webview shortcuts in your Tauri app, e.g. `F3` or `Ctrl+J`.

## Install

Install the plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-plugin-prevent-default = 0.1
```

## Usage

Register the plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_prevent_default::Builder::new().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

You can also use flags to determine which shortcuts the plugin should disable. By default, it will disable all of them.

```rust
use tauri_plugin_prevent_default::Flags;

fn main() {
  let prevent = tauri_plugin_prevent_default::Builder::new()
    .with_flags(Flags::CONTEXT_MENU | Flags::PRINT | Flags::DOWNLOADS)
    .build();

  tauri::Builder::default()
    .plugin(prevent)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
```

## Contributing

If you know of any other shortcuts that we can include in the plugin, please let us know!
