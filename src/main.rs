use gloo::console::log;
use yew::prelude::*;
#[function_component]
fn Header() -> Html {
    let onclick = { move |_| log!("onclick") };

    html! {
        <div id="header">
            <div class="box-name" id="u-name">
                {"name: "}<input type="text" />
                <button {onclick}>{"enter"}</button>
            </div>
        </div>
    }
}

#[function_component]
fn Footer() -> Html {
    html! {
        <div id="footer">{"footer"}</div>
    }
}

#[function_component]
fn Container() -> Html {
    html! {
        <div id="container">

        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Header/>
            <Container/>
            <Footer/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
