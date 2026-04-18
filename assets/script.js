/*ORIGIN*/
if (ORIGIN === null || window.location.origin === ORIGIN) {
  const shortcuts = new Map();
  window.addEventListener('keydown', (e) => {
    const eKey = e.key.toLowerCase();
    const set = shortcuts.get(eKey);
    if (set) {
      const flags = toFlags({
        altKey: e.altKey,
        ctrlKey: e.ctrlKey,
        metaKey: e.metaKey,
        shiftKey: e.shiftKey,
      });
      if (set.has(flags)) {
        e.preventDefault();
      }
    }
  });
  function onKey(key, options = {}) {
    const _key = key.toLowerCase();
    let set = shortcuts.get(_key);
    const flags = toFlags(options);
    if (set) {
      set.add(flags);
    } else {
      set = new Set([flags]);
      shortcuts.set(_key, set);
    }
  }
  function onPointer(name) {
    window.addEventListener(name, (e) => {
      e.preventDefault();
    });
  }
  function toFlags(options) {
    let flags = 0;
    if (options.altKey) flags |= (1 << 0);
    if (options.ctrlKey) flags |= (1 << 1);
    if (options.metaKey) flags |= (1 << 2);
    if (options.shiftKey) flags |= (1 << 3);
    return flags;
  }
  /*SCRIPT*/
}
