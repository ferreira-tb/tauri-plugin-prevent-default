const COMMANDS: &[&str] = &["keyboard", "pointer"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
