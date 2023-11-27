use yew::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <div id="header">{"header"}</div>
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
        <div id="container">{"container"}</div>
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
