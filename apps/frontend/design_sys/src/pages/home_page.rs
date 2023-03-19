use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::typography::{Tag, Typography};
use yew::prelude::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <>
            <DefaultLayout>
                <Typography value={"Welcome to Design System!"} tag={Tag::H1} />
            </DefaultLayout>
        </>
    )
}
