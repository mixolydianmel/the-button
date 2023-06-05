use chrono::{DateTime, Duration, Utc};
use gloo_timers::callback::Interval;
use surrealdb::sql::Datetime;
use yew::prelude::*;

pub enum TimerMsg {
    Tick,
    Update(DateTime<Utc>),
}

#[derive(PartialEq, Properties)]
pub struct TimerProps {
    pub endpoint: String,
}

pub struct Timer {
    duration: Duration,
    _interval: Interval,
}

impl Component for Timer {
    type Message = TimerMsg;
    type Properties = TimerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Timer {
            duration: Duration::zero(),
            _interval: Interval::new(500, move || link.send_message(TimerMsg::Tick)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimerMsg::Tick => {
                let endpoint = ctx.props().endpoint.clone();
                ctx.link().send_future(async {
                    let res = match reqwest::get(endpoint).await {
                        Ok(r) => r,
                        Err(e) => panic!("{}", e),
                    };

                    let txt = match res.text().await {
                        Ok(t) => t,
                        Err(e) => panic!("{}", e),
                    };

                    let time: DateTime<Utc> = Datetime::from(txt.replace("\"", "").as_str()).into();

                    TimerMsg::Update(time)
                });
                true
            }
            TimerMsg::Update(time) => {
                self.duration = Utc::now() - time;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p class="time">
                { format!("{} days, {} hours, {} minutes, and {} seconds",
                          self.duration.num_days(),
                          self.duration.num_hours() % 24,
                          self.duration.num_minutes() % 60,
                          self.duration.num_seconds() % 60) }
            </p>
        }
    }
}
