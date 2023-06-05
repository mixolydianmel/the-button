use chrono::Duration;
use gloo_timers::callback::Interval;
use yew::prelude::*;

pub enum HighScoreMsg {
    Tick,
    Update(Duration),
}

#[derive(PartialEq, Properties)]
pub struct HighScoreProps {
    pub name: String,
    pub endpoint: String,
}

pub struct HighScore {
    _interval: Interval,
    duration: Duration,
}

impl Component for HighScore {
    type Message = HighScoreMsg;
    type Properties = HighScoreProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        HighScore {
            _interval: Interval::new(10 * 1_100, move || link.send_message(HighScoreMsg::Tick)),
            duration: Duration::zero(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HighScoreMsg::Tick => {
                let endpoint = ctx.props().endpoint.clone();
                ctx.link().send_future(async {
                    let res = match reqwest::get(endpoint).await {
                        Ok(r) => r,
                        Err(e) => panic!("{}", e),
                    };

                    let duration = match res.text().await {
                        Ok(t) => t,
                        Err(e) => panic!("{}", e),
                    }
                    .replace('"', "");

                    let duration: Duration = Duration::seconds(duration.parse().unwrap_or(0));

                    HighScoreMsg::Update(duration)
                });
                true
            }
            HighScoreMsg::Update(d) => {
                self.duration = d;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        ctx.link().send_message(HighScoreMsg::Tick);
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let d = self.duration;
        let n = &ctx.props().name;
        html! {
            <div class={ "" }>
                <h3>{ n }</h3>
                <h2>
                {
                    format!("{} days, {} hours, {} minutes, and {} seconds",
                        d.num_days(),
                        d.num_hours() % 24,
                        d.num_minutes() % 60,
                        d.num_seconds() % 60)
                }
                </h2>
            </div>
        }
    }
}
