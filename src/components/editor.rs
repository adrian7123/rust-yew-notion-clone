use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use super::editor_content::EditorContent;

#[function_component]
pub fn Editor() -> Html {
    let click = Callback::from(|_| {
        let object = JsValue::from("world");
        log!("Hello", object);
    });

    html! {
        <div
            class="bg-zing-900 max-w-[1100px] mx-auto rounded-xl min-h-[500px] shadow-sm border border-black/20 overflow-hidden grid grid-cols-[16rem_1fr]">
            <aside class="bg-zinc-700 border-r border-r-zinc-700 p-4">
                <div class="flex gap-2 group ">
                    <button onclick={click.clone()} class="w-3 h-3 rounded-full bg-zinc-300 group-hover:bg-red-400">
                    </button>
                    <button onclick={click.clone()} class="w-3 h-3 rounded-full bg-zinc-300 group-hover:bg-yellow-400">
                    </button>
                    <button onclick={Callback::from(|_|teste("maximizar"))} class="w-3 h-3 rounded-full bg-zinc-300 group-hover:bg-green-400">
                    </button>
                </div>
            </aside>
            <EditorContent/>
        </div>
    }
}

fn teste(button_str: &str) {
    log!(button_str);
}
