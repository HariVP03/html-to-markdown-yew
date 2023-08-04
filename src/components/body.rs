use yew::prelude::*;

#[function_component(Body)]
pub fn body() -> Html {
    html! {
        <div class={"flex w-screen justify-around items-center h-[calc(100vh-100px)] bg-purple-100"}>
            <div class={"flex items-center justify-center w-[40%] bg-red-200 flex-col p-4 rounded-lg"}>
                <h1 class={"mb-4 text-xl"}>{"Enter Html Here"}</h1>
                <textarea class={"h-[500px] font-mono w-full"} />
            </div>

            <div class={"w-[40%] items-center bg-red-200 flex flex-col p-4"}>
                <h1 class={"mb-4 text-xl"}>{"Output Markdown Here"}</h1>
                <textarea readonly={true} class={"h-[500px] font-mono w-full"} />
            </div>
        </div>
    }
}
