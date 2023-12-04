use gloo::console::log;
// use yew::prelude::*;
use yew::{html, Component, Context, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ContainerProp {
    pub username: String,
}

pub struct ContainerComponent {}
// impl ContainerComponent {}

impl Component for ContainerComponent {
    type Message = ();
    type Properties = ContainerProp;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        log!("xxx>", ctx.props().username.to_string());
        html! {
            <>
                <div id="container">
                {ctx.props().username.to_string()}
                </div>
            </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn prepare_state(&self) -> Option<String> {
        None
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {}
}
