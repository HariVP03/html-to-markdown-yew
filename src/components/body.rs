use yew::prelude::*;
use web_sys::{ EventTarget, HtmlInputElement };
use wasm_bindgen::JsCast;

#[function_component(Body)]
pub fn body() -> Html {
    let html_handle: UseStateHandle<String> = use_state(String::default);
    let html: String = (*html_handle).clone();

    let onchange: Callback<Event> = {
        let html_handle: UseStateHandle<String> = html_handle.clone();

        Callback::from(move |e: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = e.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement
            let input: Option<_> = target.and_then(|t: EventTarget|
                t.dyn_into::<HtmlInputElement>().ok()
            );

            if let Some(input) = input {
                html_handle.set(input.value());
            }
        })
    };

    html! {
        <div class={"flex w-screen justify-around items-center h-[calc(100vh-100px)] bg-purple-100"}>
            <div class={"flex items-center justify-center w-[40%] bg-red-200 flex-col p-4 rounded-lg"}>
                <h1 class={"mb-4 text-xl"}>{"Enter Html Here"}</h1>
                <textarea value={html.clone()} onchange={onchange} class={"h-[500px] font-mono w-full"} />
            </div>

            <div class={"w-[40%] items-center bg-red-200 flex flex-col p-4"}>
                <h1 class={"mb-4 text-xl"}>{"Output Markdown Here"}</h1>
                <textarea value={html} readonly={true} class={"h-[500px] font-mono w-full"} />
            </div>
        </div>
    }
}
