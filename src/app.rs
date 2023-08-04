use yew::prelude::*;

use crate::components::{ body, header };

#[function_component(App)]
pub fn app() -> Html {
    html! {
    <div class={"max-h-screen max-w-screen overflow-hidden"}>
        <header::Header />
        <body::Body />
    </div>
    }
}
