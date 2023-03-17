use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            {"hello Design Sys."}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
