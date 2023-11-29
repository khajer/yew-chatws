use yew::prelude::*;
mod components;

use components::container::Container;
use components::footer::Footer;
use components::header::Header;

#[function_component(App)]
pub fn app() -> Html {
    html! {

        <div>
            <Header/>
            <Container/>
            <Footer/>
        </div>
    }
}
