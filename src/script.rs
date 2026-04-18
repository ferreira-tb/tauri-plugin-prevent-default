use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

/// Script to be injected into the webview.
#[derive(Clone, Debug)]
pub struct Script(Arc<str>);

impl Script {
  #[must_use]
  pub fn new(script: impl AsRef<str>) -> Self {
    Self(Arc::from(script.as_ref()))
  }

  #[must_use]
  pub fn join(&self, script: impl AsRef<str>) -> Self {
    let mut buf = String::from(self.0.as_ref());
    buf.push('\n');
    buf.push_str(script.as_ref());
    Self::from(buf)
  }
}

impl AsRef<str> for Script {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl Deref for Script {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}

impl fmt::Display for Script {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
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
