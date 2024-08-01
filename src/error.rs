use serde::{Serialize, Serializer};

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Strum(#[from] strum::ParseError),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_str())
  }
}
