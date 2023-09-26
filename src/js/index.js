export function getSelection() {
  console.log(window.getSelection());

  let selection = window.getSelection();

  let x = 0;
  let y = 0;

  if (selection.baseNode) {
    if (selection.rangeCount > 0) {
      const range = selection.getRangeAt(0);
      const rect = range.getBoundingClientRect();
      x = rect.left;
      y = rect.top + window.scrollY;
    }
  }

  return JSON.stringify([x, y]);
}

export function isSelection() {
  return (
    window.getSelection().toString().length > 0 &&
    window.getSelection().type === "Range"
  );
}
