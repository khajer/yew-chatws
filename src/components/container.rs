use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContainerProp {
    pub username: String,
}
#[function_component]
pub fn Container(prop: &ContainerProp) -> Html {
    html! {
        <>
            <div id="container">
                {prop.username.to_string()}

            </div>
        </>
    }
}
