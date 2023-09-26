use yew::prelude::*;

use super::bubble_menu_button::BubbleMenuButton;

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

        <BubbleMenuButton>
          {"Text"}
          <i class="fa-solid fa-chevron-down"></i>
        </BubbleMenuButton>
        <BubbleMenuButton>
          <i class="fa-regular fa-comment"></i>
          {"Comment"}
        </BubbleMenuButton>
        <div class="flex items-center" >
          <BubbleMenuButton command="Bold">
            <i class="w-4 h-4 fa-solid fa-bold"></i>
          </BubbleMenuButton>
          <BubbleMenuButton command="Italic">
            <i class="w-4 h-4 fa-solid fa-italic"></i>
          </BubbleMenuButton>
          <BubbleMenuButton command="Bold">
            <i class="w-4 h-4 fa-solid fa-strikethrough"></i>
          </BubbleMenuButton>
          <BubbleMenuButton command="Bold">
            <i class="w-4 h-4 fa-solid fa-code"></i>
          </BubbleMenuButton>
        </div>
      </div>
    }
}
