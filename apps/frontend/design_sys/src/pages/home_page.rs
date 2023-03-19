use crate::layout::layout::DefaultLayout;
use yew::prelude::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <>
            <DefaultLayout>
                <h1>{"Getting Started"}</h1>
                <p>{"Welcome to Design System!"}</p>
            </DefaultLayout>
        </>
    )
}
