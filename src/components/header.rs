use gloo::console::log;
use gloo::utils::document;
// use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    let username_state = use_state(|| String::from(""));
    let onclick = {
        let username_state = username_state.clone();
        move |_| {
            log!("click enter");
            let div: Element = document().get_element_by_id("inputname").unwrap();
            let value_input = div.dyn_into::<HtmlInputElement>().unwrap().value();

            let username_state = username_state.clone();

            username_state.set(value_input);
        }
    };

    html! {
        <>
        <div id="header">
        if username_state.to_string().eq(""){
            <div class="box-name" id="u-name">
                <input type="text" id="inputname" />
                <button {onclick}>{"enter"}</button>
            </div>
        }
        else{
            <div>{username_state.to_string()}</div>
        }

        </div>
        </>
    }
}
