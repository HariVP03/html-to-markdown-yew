use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class={"w-screen bg-green-100 h-[100px]"}>
            <h1 class={"text-4xl p-10 w-full bg-red-100 h-center text-center font-semibold"}>
                {"HTML ➡️  MARKDOWN PARSER"}
            </h1>
        </div>
    }
}
