mod app;
mod components;
mod context;
mod hooks;
mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
