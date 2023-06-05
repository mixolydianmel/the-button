pub mod generic;
pub mod highscore;

use generic::Generic;
use highscore::HighScore;
use yew::prelude::*;

#[function_component(Stats)]
pub fn stats() -> Html {
    html! {
        <div class="stats">
            <h1 class="stats-header"><span>{ "Stats" }</span></h1>
            <HighScore endpoint="http://0.0.0.0:8080/api/data/high_score" name="High Score"/>
            <Generic endpoint="http://0.0.0.0:8080/api/data/total_clicks" name="Presses"/>
        </div>
    }
}
