use gloo_console::log;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::EventTarget;
use yew::prelude::*;

use crate::components::bubble_menu::BubbleMenu;

#[wasm_bindgen(module = "/src/js/index.js")]
extern "C" {
    fn getSelection() -> String;
    fn isSelection() -> bool;
}

#[function_component]
pub fn EditorContent() -> Html {
    let top = use_state(|| 0.0);
    let left = use_state(|| 0.0);
    let visible = use_state(|| false);

    let on_content_change = Callback::from(|e: InputEvent| {
        let target: EventTarget = e.target().unwrap();

        log!(target);
    });

    let on_mouse_up = {
        let top = top.clone();
        let left = left.clone();
        let visible = visible.clone();
        Callback::from(move |_| {
            if isSelection() {
                visible.set(true);
                let arr: Vec<f32> = serde_json::from_str(&getSelection()).unwrap();
                log!(format!("arr: {:?}", arr));

                if let [x, y] = arr.as_slice() {
                    top.set(y - 45.0);
                    left.set(x - 22.0);
                }
            } else {
                visible.set(false);
            }

            log!(format!("x: {}; y: {}", *top, *left));
        })
    };

    html! {
      <>
        <BubbleMenu left={*left} top={*top} visible={*visible} />
        <div
            onmouseup={on_mouse_up}
            oninput={on_content_change}
            contenteditable="true"
            id="editor"
            class="w-full h-full p-2 outline-none overflow-x-auto bg-zinc-700"
            placeholder="Start typing...">
                <div class=" text-white p-4 text-center">
                    <h1 class="text-4xl font-bold">{"The Power of Tailwind CSS: Simplify Your Web Development"}</h1>
                    <p class="text-sm">{"Published on September 23, 2023 by Your Name"}</p>
                </div>
                <main class="max-w-4xl mx-auto p-4 mt-4 rounded shadow-md">
                    <article >
                        <p>{"If you're a web developer, you've likely encountered the challenges of writing and managing complex CSS code. Thankfully, Tailwind CSS is here to make your life easier. In this blog post, we'll explore how Tailwind CSS simplifies web development by providing a set of utility classes."}</p>

                        <h2 >{"What is Tailwind CSS?"}</h2>
                        <p>{"Tailwind CSS is a utility-first CSS framework that streamlines your workflow by offering a comprehensive set of utility classes. These classes can be directly applied to your HTML elements, allowing you to style your web pages without writing custom CSS."}</p>

                        <h2 class="text-2xl font-semibold mt-4">{"Getting Started"}</h2>
                        <p>{"Getting started with Tailwind CSS is straightforward. First, include the Tailwind CSS file in your HTML:"}</p>
                            {"<link href='https://cdn.jsdelivr.net/npm/tailwindcss@2.2.16/dist/tailwind.min.css' rel='stylesheet'>"}
                        <p>{"Now you can start using Tailwind CSS classes to style your content. For example, to create a button with a blue background and white text:"}</p>

                        <h2 class="text-2xl font-semibold mt-4">{"Customization"}</h2>
                        <p>{"One of the great things about Tailwind CSS is its flexibility. You can customize the default styles by modifying the configuration file. For example, you can change the default color palette, fonts, and spacing."}</p>

                        <h2 class="text-2xl font-semibold mt-4">{"Conclusion"}</h2>
                        <p>{"With Tailwind CSS, web development becomes more efficient and maintainable. You can create stylish web pages with minimal effort and easily customize the framework to suit your project's needs."}</p>
                    </article>
                </main>
                <footer class=" text-white text-center p-4">
                    <p>{"&copy; 2023 Your Blog Name. All rights reserved."}</p>
                </footer>
            </div>
      </>
    }
}
