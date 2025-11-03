/*ORIGIN*/
if (window.location.origin.includes(ORIGIN ?? '')) {
  function onKey(key, options = {}) {
    const _key = key.toLowerCase();
    options.altKey ??= false;
    options.ctrlKey ??= false;
    options.metaKey ??= false;
    options.shiftKey ??= false;
    window.addEventListener('keydown', (e) => {
      if (
        e.altKey !== options.altKey ||
        e.ctrlKey !== options.ctrlKey ||
        e.metaKey !== options.metaKey ||
        e.shiftKey !== options.shiftKey
      ) {
        return;
      }
      if (e.key.toLowerCase() === _key) {
        e.preventDefault();
      }
    });
  }
  function onPointer(name) {
    window.addEventListener(name, (e) => {
      e.preventDefault();
    });
  }
  /*SCRIPT*/
}
