use crate::components::{button::Button, timer::Timer, stats::Stats};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <div class="container">
            <Button endpoint={"http://66.228.35.230:80/api/click"}/>
            <Timer endpoint={"http://66.228.35.230:80/api/latest"}/>
            <footer>
                { "Made with " }
                <i class="fa-solid fa-heart" style="color: #ed333b;"></i>
                { " by " }
                <a href="http://github.com/mixolydianmel">{ "Melody" }</a>
            </footer>
        </div>
        <Stats/>
        </>
    }
}
