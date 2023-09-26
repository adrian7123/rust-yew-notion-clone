export function get_selection_position() {
  const selection = window.getSelection();

  let x = 0;
  let y = 0;

  if (selection.rangeCount > 0) {
    const range = selection.getRangeAt(0);
    const rect = range.getBoundingClientRect();

    console.log(selection, rect);
    x = rect.left;
    y = rect.top + window.scrollY;
  }

  return JSON.stringify([x, y]);
}

export function is_selection() {
  return (
    window.getSelection().toString().length > 0 &&
    window.getSelection().type === "Range"
  );
}

export function get_selection() {
  return window.getSelection();
}

export function exec_command(command) {
  document.execCommand(command, false, null);
}
