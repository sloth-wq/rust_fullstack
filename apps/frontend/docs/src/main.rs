mod layout;
mod pages;

use pages::app_checkbox_page::AppCheckboxPage;
use pages::app_radio_group_page::AppRadioGroupPage;
use pages::app_textarea_page::AppTextareaPage;
use pages::app_typography_page::AppTypographyPage;
use pages::home_page::Home;
use pages::stack_page::StackPage;
use pages::{
    app_box_page::AppBoxPage, app_button_page::AppButtonPage, app_input_page::AppInputPage,
};
use yew::prelude::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/ui/AppButton")]
    AppButton,
    #[at("/ui/AppTypography")]
    AppTypography,
    #[at("/ui/Stack")]
    Stack,
    #[at("/ui/AppBox")]
    AppBox,
    #[at("/ui/AppInput")]
    AppInput,
    #[at("/ui/AppTextarea")]
    AppTextarea,
    #[at("/ui/AppCheckbox")]
    AppCheckbox,
    #[at("/ui/AppRadioGroup")]
    AppRadioGroup,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::AppButton => html! { <AppButtonPage /> },
        Route::AppTypography => html! { <AppTypographyPage /> },
        Route::Stack => html! { <StackPage /> },
        Route::AppBox => html! { <AppBoxPage /> },
        Route::AppInput => html! { <AppInputPage /> },
        Route::AppTextarea => html! { <AppTextareaPage /> },
        Route::AppCheckbox => html! { <AppCheckboxPage /> },
        Route::AppRadioGroup => html! { <AppRadioGroupPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
