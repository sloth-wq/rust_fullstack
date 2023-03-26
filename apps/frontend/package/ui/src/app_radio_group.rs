use stylist::style;
use yew::html::onchange::Event;
use yew::prelude::{html, Html, Properties};
use yew::{function_component, Callback};

#[derive(PartialEq, Clone, Debug)]
pub struct Item {
    pub label: String,
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub item_list: Vec<Item>,
    #[prop_or_default]
    pub is_error: bool,
    pub on_change: Callback<Event>,
}

// TODO: エラー時に radio の枠を赤くする.

#[function_component(AppRadioButtonGroup)]
pub fn app_radio_button_group(
    Props {
        name,
        item_list,
        is_error,
        on_change,
    }: &Props,
) -> Html {
    let contaienr_style = style!(
        r#"
        display: flex;
        flex-direction: column;
    "#
    )
    .unwrap();

    let checkbox_style = style!(
        r#"
            accent-color: #003EE5;
            margin: 4px 16px 4px 4px;
            width: 16px;
            height: 16px;
        "#
    )
    .unwrap();

    let label_style = style!(
        r#"
        margin: 0;
        display: flex;
        width: 100%;
        align-items: center;
        min-height: 44px;
    "#
    )
    .unwrap();

    html! {
        <div class={contaienr_style}>
            {item_list.iter().map(| Item { label, value, checked, disabled }| html! {
                <label class={label_style.clone()} for={value.clone()}>
                    <input
                        type="radio"
                        name={name.clone()}
                        class={checkbox_style.clone()}
                        id={value.clone()}
                        value={value.clone()}
                        checked={*checked}
                        disabled={*disabled}
                        onchange={on_change.clone()}
                    />
                    <span style={if *disabled { "color: #949497;" } else if *is_error { "color: red;" } else { "" }}>
                        {label.clone()}
                    </span>
                </label>
        }).collect::<Html>()}
        </div>
    }
}

#[function_component(AppRadioCardGroup)]
pub fn app_radio_card_group(
    Props {
        name,
        is_error,
        item_list,
        on_change,
    }: &Props,
) -> Html {
    let contaienr_style = style!(
        r#"
        display: flex;
        flex-direction: column;
        gap: 8px;
    "#
    )
    .unwrap();

    let checkbox_style = style!(
        r#"
            accent-color: #003EE5;
            margin: 4px 16px 4px 16px;
            width: 16px;
            height: 16px;
        "#
    )
    .unwrap();

    let label_style = style!(
        r#"
        margin: 0;
        display: flex;
        width: 100%;
        align-items: center;
        min-height: 44px;
        border-radius: 4px;
    "#
    )
    .unwrap();

    html! {
        <div class={contaienr_style}>
            {item_list.iter().map(| Item { label, value, checked, disabled }| html! {
                <label
                    class={label_style.clone()}
                    for={value.clone()}
                    style={if *disabled { "border: 1px solid #949497;" } else if *is_error { "border: 1px solid red;" } else { "border: 1px solid black;" } }
                >
                    <input
                        type="radio"
                        name={name.clone()}
                        class={checkbox_style.clone()}
                        id={value.clone()}
                        value={value.clone()}
                        checked={*checked}
                        disabled={*disabled}
                        onchange={on_change.clone()}
                    />
                    <span style={if *disabled { "color: #949497;" } else if *is_error { "color: red;" } else { "" }}>
                        {label.clone()}
                    </span>
                </label>
        }).collect::<Html>()}
        </div>
    }
}
