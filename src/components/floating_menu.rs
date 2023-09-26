use yew::prelude::*;

use crate::components::floating_menu_button::FloatingMenuButton;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub top: f32,
    pub left: f32,
    #[prop_or(false)]
    pub visible: bool,
}

#[function_component]
pub fn FloatingMenu(props: &Props) -> Html {
    html! {
      <div class={classes!(
        format!(
          "{} absolute outline-none gap-1 flex-col py-2 px-1 bg-zinc-700 shadow-xl border border-zinc-600 shadow-black/20 rounded-lg overflow-hidden divide-x divide-zinc-600",
          if props.visible {"flex"} else {"hidden"}
        )
        )}
        style={format!("top:{}px;left:{}px;",props.top,props.left)}>

        <FloatingMenuButton>
          <img class="w-12 border border-zinc-600 rounded" src="http://www.notion.so/images/blocks/text/en-US.png" alt=""/>
          <div class="flex flex-col text-left">
            <span class="text-sm">{"Texto"}</span>
            <span class="text-xs text-zinc-400">{"Comece a escrever com texto sem formatação."}</span>
          </div>
        </FloatingMenuButton>
        <FloatingMenuButton>
          <img class="w-12 border border-zinc-600 rounded" src="http://www.notion.so/images/blocks/header.57a7576a.png" alt=""/>
          <div class="flex flex-col text-left">
            <span class="text-sm">{"Título 1"}</span>
            <span class="text-xs text-zinc-400">{"Título de seção grande."}</span>
          </div>
        </FloatingMenuButton>
      </div>
    }
}
