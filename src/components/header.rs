use gloo::console::log;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropHeaderProp {
    pub on_login: Callback<String>,
}

#[function_component]
pub fn Header(prop: &PropHeaderProp) -> Html {
    let username_state = use_state(|| String::from(""));
    let onclick = {
        let username_state = username_state.clone();
        let on_login = prop.on_login.clone();
        move |_| {
            log!("click enter");
            let div: Element = document().get_element_by_id("input_name").unwrap();
            let value_input = div.dyn_into::<HtmlInputElement>().unwrap().value();
            let username_state = username_state.clone();
            on_login.emit(value_input.clone());
            username_state.set(value_input);
        }
    };

    html! {
        <>
        <div id="header">
        if username_state.to_string().eq(""){
            <div class="box-name" id="u-name">
                <input type="text" id="input_name" />
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
