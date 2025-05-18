use std::fmt;
use std::ops::{Add, AddAssign};
use std::sync::Arc;

/// Script to be injected into the webview.
#[derive(Debug)]
pub struct Script(Arc<str>);

impl Script {
  pub fn new(script: impl AsRef<str>) -> Self {
    Self(Arc::from(script.as_ref()))
  }

  #[must_use]
  pub fn join(&self, script: impl AsRef<str>) -> Self {
    let script = script.as_ref();
    let capacity = self.0.len().saturating_add(script.len());
    let mut buf = String::with_capacity(capacity);
    buf.push_str(&self.0);
    buf.push('\n');
    buf.push_str(script);
    Self::from(buf)
  }
}

impl AsRef<str> for Script {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Clone for Script {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}

impl From<&str> for Script {
  fn from(value: &str) -> Self {
    Script(Arc::from(value))
  }
}

impl From<String> for Script {
  fn from(value: String) -> Self {
    Script(Arc::from(value))
  }
}

impl From<&Script> for String {
  fn from(value: &Script) -> Self {
    String::from(value.0.as_ref())
  }
}

impl From<Script> for String {
  fn from(value: Script) -> Self {
    <Self as From<&Script>>::from(&value)
  }
}

impl From<&Script> for Vec<u8> {
  fn from(value: &Script) -> Self {
    value.0.as_bytes().to_vec()
  }
}

impl From<Script> for Vec<u8> {
  fn from(value: Script) -> Self {
    <Self as From<&Script>>::from(&value)
  }
}

impl From<&Script> for Vec<u16> {
  fn from(value: &Script) -> Self {
    value
      .0
      .as_bytes()
      .iter()
      .copied()
      .map(u16::from)
      .collect()
  }
}

impl From<Script> for Vec<u16> {
  fn from(value: Script) -> Self {
    <Self as From<&Script>>::from(&value)
  }
}

impl<T: AsRef<str>> Add<T> for Script {
  type Output = Script;

  fn add(self, rhs: T) -> Self::Output {
    self.join(rhs)
  }
}

impl<T: AsRef<str>> AddAssign<T> for Script {
  fn add_assign(&mut self, other: T) {
    *self = self.join(other);
  }
}

impl fmt::Display for Script {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}
