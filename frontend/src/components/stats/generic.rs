use gloo_timers::callback::Interval;
use yew::prelude::*;

pub enum GenericMsg {
    Tick,
    Update(usize),
}

#[derive(PartialEq, Properties)]
pub struct GenericProps {
    pub name: String,
    pub endpoint: String,
}

pub struct Generic {
    _interval: Interval,
    count: usize,
}

impl Component for Generic {
    type Message = GenericMsg;
    type Properties = GenericProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        Generic {
            _interval: Interval::new(10 * 1_100, move || link.send_message(GenericMsg::Tick)),
            count: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GenericMsg::Tick => {
                let endpoint = ctx.props().endpoint.clone();
                ctx.link().send_future(async {
                    let res = match reqwest::get(endpoint).await {
                        Ok(r) => r,
                        Err(e) => panic!("{}", e),
                    };

                    let count: usize = match res.text().await {
                        Ok(t) => t,
                        Err(e) => panic!("{}", e),
                    }
                    .replace('"', "")
                    .as_str()
                    .parse()
                    .unwrap_or(69);

                    GenericMsg::Update(count)
                });
                true
            }
            GenericMsg::Update(c) => {
                self.count = c;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        ctx.link().send_message(GenericMsg::Tick);
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let c = self.count;
        let n = &ctx.props().name;
        html! {
            <div class={ format!("{} stat",n.to_lowercase().replace(' ', "-")) }>
                <h3>{ n }</h3>
                <h2>{ format!("{}", c) }</h2>
            </div>
        }
    }
}
