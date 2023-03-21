mod layout;
mod pages;

use pages::app_typography_page::AppTypographyPage;
use pages::home_page::Home;
use pages::stack_page::StackPage;
use pages::{
    app_box_page::AppBoxPage, app_button_page::AppButtonPage, string_input_page::StringInputPage,
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
    #[at("/ui/StringInput")]
    StringInput,
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
        Route::StringInput => html! { <StringInputPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
