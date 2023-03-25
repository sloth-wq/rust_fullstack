use stylist::style;
use yew::html::onchange::Event;
use yew::prelude::{html, Html, Properties};
use yew::{function_component, Callback};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: String,
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub on_change: Callback<Event>,
}

#[function_component(AppCheckbox)]
pub fn app_checkbox(
    Props {
        label,
        value,
        checked,
        disabled,
        on_change,
    }: &Props,
) -> Html {
    let contaienr_style = style!(
        r#"
        display: flex;
    "#
    )
    .unwrap();

    let checkbox_style = style!(
        r#"
            accent-color: #003EE5;
            margin: 4px;
            width: 16px;
            height: 16px;
        "#
    )
    .unwrap();

    let label_style = style!(
        r#"
        margin: 0;
        display: block;
        padding-left: 12px;
    "#
    )
    .unwrap();

    html! {
        <div class={contaienr_style}>
            <input
                class={checkbox_style}
                id={value.clone()}
                value={value.clone()}
                checked={*checked}
                disabled={*disabled}
                onchange={on_change.clone()}
                type="checkbox"
            />
            <label class={vec!(label_style)} for={value.clone()}>{label.clone()}</label>
        </div>
    }
}
