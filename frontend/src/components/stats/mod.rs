pub mod generic;
pub mod time;

use generic::Generic;
use time::Time;
use yew::prelude::*;

#[function_component(Stats)]
pub fn stats() -> Html {
    html! {
        <div class="stats">
            <h1 class="stats-header"><span>{ "Stats" }</span></h1>
            <Time endpoint="http://0.0.0.0:8080/api/data/prev_time" name="Previous Time"/>
            <Time endpoint="http://66.228.35.230:80/api/data/high_score" name="High Score"/>
            <Generic endpoint="http://66.228.35.230:80/api/data/total_clicks" name="Presses"/>
        </div>
    }
}
