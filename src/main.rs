use yew::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <div id="header">

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
            <div class="box-name" id="u-name">
                {"name: "}<input type="text" />
                <input type="button" value="enter" />
            </div>
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
