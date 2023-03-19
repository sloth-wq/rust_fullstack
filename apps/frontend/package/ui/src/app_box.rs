use yew::prelude::{html, Html, Properties};
use yew::{function_component, Children};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(AppBox)]
pub fn app_box(Props { children }: &Props) -> Html {
    html! {
        <>
            <div>
                { for children.iter() }
            </div>
        </>
    }
}
