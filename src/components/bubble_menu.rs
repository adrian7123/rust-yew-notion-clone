use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub top: f32,
    pub left: f32,
    #[prop_or_default]
    pub visible: bool,
}

#[function_component]
pub fn BubbleMenu(props: &Props) -> Html {
    html! {
      <div class={classes!(
        format!(
          "{} absolute bg-zinc-700 shadow-xl border border-zinc-600 shadow-black/20 rounded-lg overflow-hidden divide-x divide-zinc-600",
          if props.visible {"flex"} else {"hidden"}
        ))}
        style={format!("top:{}px;left:{}px;",props.top,props.left)}>
        <button class="p-2 text-zinc-300 text-sm flex items-center gap-1.5 font-medium leading-none">
          <object class="w-5 h-5 text-black" data="/assets/icons/bold.svg"></object>
        </button>
        <button class="p-2 text-zinc-300 text-sm flex items-center gap-1.5 font-medium leading-none">{"Bold"}</button>
        <button class="p-2 text-zinc-300 text-sm flex items-center gap-1.5 font-medium leading-none">{"Bold"}</button>
        <button class="p-2 text-zinc-300 text-sm flex items-center gap-1.5 font-medium leading-none">{"Bold"}</button>
        <button class="p-2 text-zinc-300 text-sm flex items-center gap-1.5 font-medium leading-none">{"Bold"}</button>
      </div>
    }
}
