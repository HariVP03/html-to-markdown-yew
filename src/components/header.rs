use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class={"w-screen h-screen bg-green-100"}>
            <h1 class={"text-4xl p-10 w-full bg-red-100 text-center font-semibold"}>
                {"HTML ➡️  MARKDOWN PARSER"}
            </h1>
        </div>
    }
}
