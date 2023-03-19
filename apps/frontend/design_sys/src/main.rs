mod layout;
mod pages;

use pages::app_button_page::AppButtonPage;
use pages::home_page::Home;
use pages::typography_page::TypographyPage;
use yew::prelude::{function_component, html, Html};
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/ui/AppButton")]
    AppButton,
    #[at("/ui/Typography")]
    Typography,
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
        Route::Typography => html! { <TypographyPage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
