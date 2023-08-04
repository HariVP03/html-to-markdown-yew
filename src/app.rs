use yew::prelude::*;

use crate::components::header;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <header::Header />
    }
}
