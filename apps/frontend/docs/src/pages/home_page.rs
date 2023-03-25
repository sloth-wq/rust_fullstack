use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_typography::{AppTypography, Tag};
use yew::prelude::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <>
            <DefaultLayout>
                <AppTypography value={"Welcome to Docs!"} tag={Tag::H1} />
            </DefaultLayout>
        </>
    )
}
