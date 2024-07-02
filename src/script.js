(() => {
  function createPredicate(key) {
    if (Array.isArray(key)) return (e) => key.includes(e.key);
    return (e) => e.key === key;
  }

  function onKey(key, options = {}) {
    const predicate = createPredicate(key);
    const { altKey = false, ctrlKey = false, shiftKey = false } = options;

    globalThis.addEventListener("keydown", (e) => {
      if (
        e.altKey !== altKey ||
        e.ctrlKey !== ctrlKey ||
        e.shiftKey !== shiftKey
      ) {
        return;
      }

      if (predicate(e)) {
        e.preventDefault();
      }
    });
  }

  //REPLACE_ME
})();
