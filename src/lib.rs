use yew::prelude::*;
mod components;
mod services;

use gloo::console::log;

use components::container::ContainerComponent;
use components::footer::Footer;
use components::header::Header;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);

    let on_login = {
        let user_state = user_state.clone();

        Callback::from(move |name: String| {
            log!(format!("name {}", name));
            user_state.set(User { username: name });
            services::ws_service::connect_websocket();
        })
    };

    html! {
        <div>
            <Header {on_login}/>
            <ContainerComponent username={user_state.username.to_string()}/>
            <Footer/>
        </div>
    }
}
