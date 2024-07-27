/*EMIT*/ /*ORIGIN*/
if (window.location.origin.includes(ORIGIN ?? '')) {
  const invoke = window.__TAURI_INTERNALS__?.invoke;
  if (typeof invoke !== 'function') {
    throw new TypeError('tauri invoke function not found');
  }
  function onKey(key, options = {}) {
    const _key = key.toLowerCase();
    options.altKey ??= false;
    options.ctrlKey ??= false;
    options.shiftKey ??= false;
    window.addEventListener('keydown', (e) => {
      if (
        e.altKey !== options.altKey ||
        e.ctrlKey !== options.ctrlKey ||
        e.shiftKey !== options.shiftKey
      ) {
        return;
      }
      if (e.key.toLowerCase() === _key) {
        e.preventDefault();
        if (EMIT) {
          const payload = { key, ...options };
          invoke('plugin:prevent-default|keyboard', { payload });
        }
      }
    });
  }
  function onPointer(event) {
    window.addEventListener(event, (e) => {
      e.preventDefault();
      if (EMIT) {
        const payload = { event };
        invoke('plugin:prevent-default|pointer', { payload });
      }
    });
  }
  /*SCRIPT*/
}
