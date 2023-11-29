use gloo::console::log;
use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
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
