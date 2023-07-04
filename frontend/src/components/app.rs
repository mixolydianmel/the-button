use crate::components::{button::Button, timer::Timer, stats::Stats};
use yew::prelude::*;

// NOTE: OLD
// #[function_component(App)]
// pub fn app() -> Html {
//     html! {
//         <>
//         <div class="container">
//             <Button endpoint={"http://66.228.35.230:80/api/click"}/>
//             <Timer endpoint={"http://66.228.35.230:80/api/latest"}/>
//             <footer>
//                 { "Made with " }
//                 <i class="fa-solid fa-heart" style="color: #ed333b;"></i>
//                 { " by " }
//                 <a href="http://github.com/mixolydianmel">{ "Melody" }</a>
//             </footer>
//         </div>
//         <Stats/>
//         </>
//     }
// }

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <h1>{ "So long" }</h1>
            <h3>{ "It's been a wild ride. Check out the project on " } <a href="http://github.com/mixolydianmel/the-button">{ "github" }</a>{ "." }</h3>
        </div>
    }
}
