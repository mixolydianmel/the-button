use yew::prelude::*;

pub enum ButtonMsg {
    Click,
    Success,
}

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub endpoint: String,
}

pub struct Button;

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Button
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ButtonMsg::Click => {
                let client = reqwest::Client::new();
                let endpoint = ctx.props().endpoint.clone();
                ctx.link().send_future(async move {
                    client.post(endpoint).send().await.unwrap();
                    ButtonMsg::Success
                });
                true
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| ButtonMsg::Click);
        html! {
            <button {onclick} class="the-button"></button>
        }
    }
}
