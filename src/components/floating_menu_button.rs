use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or("h1".to_string())]
    pub tag: String,
}

#[function_component]
pub fn FloatingMenuButton(props: &Props) -> Html {
    let click = Callback::from(|_| {});

    html! {
      <button tabindex="2" onclick={&click} class="flex items-center gap-2 p-1 rounded min-w-[200px] hover:bg-zinc-600">
        {props.children.clone()}
      </button>
    }
}
