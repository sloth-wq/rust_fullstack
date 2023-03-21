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
    pub _type: Option<Type>,
    pub disabled: Option<bool>,
    pub required: Option<bool>,
    pub is_error: Option<bool>,
    pub placeholder: Option<String>,
    pub on_input: Callback<InputEvent>,
}

#[function_component(AppInput)]
pub fn app_input(
    Props {
        _type,
        disabled,
        required,
        is_error,
        placeholder,
        on_input,
    }: &Props,
) -> Html {
    let style = style!(
        r#"
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        background: transparent;
        border: #19191C 1.5px solid;
        border-radius: 4px;
        font-size: 16px;
        outline: #19191C;
        padding: 8px;
        :disabled {
            border: #949497 1.5px solid;
            outline: #949497;
            cursor: not-allowed;
        }
        :focus {
            border: #D18E0F 1.5px solid;
            outline: #D18E0F;
        }
    "#
    )
    .unwrap();

    let __type = match _type {
        Some(v) => v,
        None => &Type::Text,
    };

    let _disabled = match disabled {
        Some(v) => v,
        None => &false,
    };

    let _required = match required {
        Some(v) => v,
        None => &false,
    };

    let error_style = match is_error {
        Some(v) => match v {
            &true => style!(
                r#"
                border: red 1.5px solid;
                outline: red;
            "#
            )
            .unwrap(),
            &false => style!(r#""#).unwrap(),
        },
        None => style!(r#""#).unwrap(),
    };

    html! {
        <input
            class={vec!(error_style, style)}
            type={__type.to_string()}
            disabled={*_disabled}
            required={*_required}
            placeholder={placeholder.clone()}
            oninput={on_input.clone()}
        />
    }
}
