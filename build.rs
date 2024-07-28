const COMMANDS: &[&str] = &["keyboard", "pointer"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();

  /*
  let readme = std::fs::read_to_string("README.md").unwrap();
  let lines = readme
    .lines()
    .map(|line| format!("//! {line}"))
    .collect::<Vec<_>>()
    .join("\n");

  let src = std::fs::read_to_string("src/lib.rs").unwrap();
  std::fs::write("src/lib.rs", format!("{lines}\n{src}")).unwrap();
  */
}
