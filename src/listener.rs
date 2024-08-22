use std::sync::atomic::{self, AtomicUsize};
use std::sync::Arc;
use std::{fmt, hash};
use tauri::{Runtime, Window};

static LISTENER_ID: AtomicUsize = AtomicUsize::new(0);

type EventListenerFn<R> = dyn Fn(&Window<R>) + Send + Sync;

pub struct EventListener<R: Runtime> {
  id: usize,
  inner: Arc<EventListenerFn<R>>,
}

impl<R: Runtime> EventListener<R> {
  pub(crate) fn new<F>(listener: F) -> Self
  where
    F: Fn(&Window<R>) + Send + Sync + 'static,
  {
    let id = LISTENER_ID.fetch_add(1, atomic::Ordering::Relaxed);
    Self { id, inner: Arc::new(listener) }
  }

  pub(crate) fn call(&self, window: &Window<R>) {
    (self.inner)(window)
  }
}

impl<R: Runtime> Clone for EventListener<R> {
  fn clone(&self) -> Self {
    Self {
      id: self.id,
      inner: Arc::clone(&self.inner),
    }
  }
}

impl<R: Runtime> PartialEq for EventListener<R> {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<R: Runtime> Eq for EventListener<R> {}

impl<R: Runtime> hash::Hash for EventListener<R> {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    self.id.hash(state)
  }
}

impl<R: Runtime> fmt::Debug for EventListener<R> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("EventListener")
      .field("id", &self.id)
      .finish_non_exhaustive()
  }
}
