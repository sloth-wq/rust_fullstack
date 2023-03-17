use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            {"hello admin."}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
