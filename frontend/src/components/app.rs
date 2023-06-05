use crate::components::{button::Button, timer::Timer};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Button endpoint={"http://66.228.35.230:80/api/click"}/>
            <Timer endpoint={"http://66.228.35.230:80/api/latest"}/>
            <footer>
                { "Made with ❤ by " }
                <a href="https://github.com/mixolydianmel">
                    { "Melody" }
                </a>
            </footer>
        </div>
    }
}
