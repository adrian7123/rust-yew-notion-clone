use wasm_bindgen::prelude::*;

use crate::models::selection::Selection;

#[wasm_bindgen(module = "/src/shared/js/commands.js")]
extern "C" {
    pub fn get_selection_position() -> String;
    pub fn is_selection() -> bool;
    pub fn get_selection() -> Selection;
    pub fn exec_command(command: String);
}
