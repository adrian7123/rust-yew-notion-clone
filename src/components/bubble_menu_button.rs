use yew::prelude::*;

use crate::shared::js_commands;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or("Bold".to_string())]
    pub command: String,
}

#[function_component]
pub fn BubbleMenuButton(props: &Props) -> Html {
    let onclick = {
        let command = props.command.clone();
        Callback::from(move |_| {
            js_commands::exec_command(command.to_string());
        })
    };

    html! {
      <button onclick={onclick} class="p-2 text-zinc-200 text-sm flex items-center gap-1.5 font-medium leading-none hover:text-zinc-50 hover:bg-zinc-600">
        {props.children.clone()}
      </button>
    }
}
