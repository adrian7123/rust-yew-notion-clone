use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub top: i32,
    pub left: i32,
}

#[function_component]
pub fn BubbleMenu(props: &Props) -> Html {
    html! {
      <div class="absolute" style={format!("top:{}px;left:{}px;",props.top,props.left)}>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">{"Bold"}</button>
      </div>
    }
}
