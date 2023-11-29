use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// pub struct Prop {
//     pub nickname: String,
// }
#[function_component(TextInput)]
pub fn input_name() -> Html {
    let onchange = Callback::from(|event: Event| {
        log!("text change");
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        log!(input.value());
    });

    let onclick = { move |_| log!("onclick") };
    html! {
        <>
        {"name: "}<input type="text" onchange={onchange} />
        <button {onclick}>{"enter"}</button>
        </>
    }
}

#[function_component]
pub fn Header() -> Html {
    let mut name = "";
    html! {
        <div id="header">
        if name == ""{
            <div class="box-name" id="u-name">
                <TextInput />

            </div>
        }
        else{
            <div>{name}</div>
        }

        </div>
    }
}
