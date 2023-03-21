use strum_macros::{self, Display};
use stylist::style;
use yew::prelude::{html, Html, Properties};
use yew::{function_component, Callback, InputEvent};

#[derive(PartialEq, Display)]
pub enum Type {
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "number")]
    Number,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub is_error: bool,
    #[prop_or_default]
    pub placeholder: String,
    pub on_input: Callback<InputEvent>,
}

#[function_component(AppTextarea)]
pub fn app_textarea(
    Props {
        value,
        disabled,
        required,
        is_error,
        placeholder,
        on_input,
    }: &Props,
) -> Html {
    let style = style!(
        r#"
        resize: none;
        min-width: 300px;
        min-height: 200px;
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        background: transparent;
        border: #19191C 1.5px solid;
        border-radius: 4px;
        font-size: 16px;
        outline: #19191C;
        padding: 8px;
        ::placeholder {
            color: #757578;
        }
        :disabled {
            border: #949497 1.5px solid;
            outline: #949497;
            cursor: not-allowed;
            color: #949497;
        }
        :focus {
            border: #D18E0F 1.5px solid;
            outline: #D18E0F;
        }
    "#
    )
    .unwrap();

    let error_style = match is_error {
        &true => style!(
            r#"
            border: red 1.5px solid;
            outline: red;
        "#
        )
        .unwrap(),
        &false => style!(r#""#).unwrap(),
    };

    html! {
        <textarea
            class={vec!(error_style, style)}
            value={value.clone()}
            disabled={*disabled}
            required={*required}
            placeholder={placeholder.clone()}
            oninput={on_input.clone()}
        />
    }
}
