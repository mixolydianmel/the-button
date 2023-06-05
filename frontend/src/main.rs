pub mod components;
pub mod model;

use components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
