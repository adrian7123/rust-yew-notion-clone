mod app;
mod components;
mod models;
mod shared;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
