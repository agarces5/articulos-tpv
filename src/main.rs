mod app;
mod components;
mod hooks;
mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
