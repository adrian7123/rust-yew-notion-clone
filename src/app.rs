use yew::prelude::*;

use crate::components::editor::Editor;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="min-h-screen p-8 text-zinc-50 bg-gradient-to-r from-pink-300 via-purple-300 to-indigo-400">
            <Editor/>
        </div>
    }
}
