mod layout;
mod pages;

use pages::app_button_page::AppButtonPage;
use pages::app_typography_page::AppTypographyPage;
use pages::home_page::Home;
use pages::stack_page::StackPage;
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
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
