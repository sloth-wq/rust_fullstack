use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            {"hello employee."}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
