use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlElement, Node};

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = Node , extends = js_sys :: Object , js_name = Selection , typescript_type = "Selection")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Selection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
    pub type Selection;

    #[wasm_bindgen(method, getter, js_class = "Selection", js_name = anchorNode)]
    pub fn anchor_node(this: &Selection) -> Node;
    #[wasm_bindgen(method, getter, js_class = "Selection", js_name = baseNode)]
    pub fn base_node(this: &Selection) -> Node;
    #[wasm_bindgen(method, getter, js_class = "Selection", js_name = focusNode)]
    pub fn focus_node(this: &Selection) -> Node;
    #[wasm_bindgen(method, getter, js_class = "Selection", js_name = focusNode)]
    pub fn focus_node_html(this: &Selection) -> HtmlElement;
}
