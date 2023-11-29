use gloo::console::log;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub name: String,
    pub name_change: Callback<User>,
}

#[derive(Default, Clone)]
pub struct User {
    pub name: String,
}

#[function_component(TextInput)]
pub fn input_name(prop: &Prop) -> Html {
    let onchange = Callback::from(|event: Event| {
        log!("text change");
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        log!(input.value());
    });

    let _namechange_fn = prop.name_change.clone();

    let onclick = {
        move |_| {
            log!("onclick ->> ");
            let div: Element = document().get_element_by_id("inputname").unwrap();
            let value_input = div.dyn_into::<HtmlInputElement>().unwrap().value();
            log!(&value_input);

            if &value_input != "" {
                let user = User {
                    name: "xxx".to_string(),
                };
                _namechange_fn.emit(user);
                log!("name set up ");
            } else {
                log!("name not setup");
            }
        }
    };

    html! {
        <>
        {"name: "}<input id="inputname" name = {prop.name.clone()} type="text" onchange={onchange} />
        <button {onclick}>{"enter"}</button>
        </>
    }
}

#[function_component]
pub fn Header() -> Html {
    let mut name = String::new();
    let name_change = Callback::from(|user: User| {
        log!(">>>> {}", user.name);
        // name = "test"
        log!("root control");
    });
    html! {
        <div id="header">
        if name == ""{
            <div class="box-name" id="u-name">
                <TextInput name={name} {name_change}/>

            </div>
        }
        else{
            <div>{name}</div>
        }

        </div>
    }
}
