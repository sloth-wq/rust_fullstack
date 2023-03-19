use stylist::{style, yew::styled_component, Style};
use yew::prelude::{html, Callback, Html, Properties};
use yew::MouseEvent;

#[derive(PartialEq, Debug)]
pub enum Size {
    Large,
    Medium,
    Small,
}

impl Size {
    pub fn style(&self) -> Result<Style, stylist::Error> {
        match &self {
            Size::Large => style!(
                r#"
                padding: 8px 32px;
            "#
            ),
            Size::Medium => style!(
                r#"
                padding: 8px 24px;
            "#
            ),
            Size::Small => style!(
                r#"
                padding: 8px 16px;
            "#
            ),
        }
    }
}

#[derive(PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
    Tertialy,
}

impl Variant {
    pub fn style(&self) -> Result<Style, stylist::Error> {
        match &self {
            Variant::Primary => style!(
                r#"
                color: #FFFFFF;
                background-color: #003EE5;
                border: #003EE5 1px solid;
                :hover {
                    background-color: #0030B2;
                }
                :active {
                    background-color: #003EE5;
                    border: #D18E0F 1px solid;
                }
                :disabled {
                    background-color: #949497;
                    border: #949497 1px solid;
                }
            "#
            ),
            Variant::Secondary => style!(
                r#"
                color: #003EE5;
                border: #003EE5 1px solid;
                background-color: #F1F1F4;
                :hover {
                    color: #0030B2;
                    background-color: #E8F1FE;
                }
                :active {
                    color: #0030B2;
                    background-color: #E8F1FE;
                    border: #D18E0F 1px solid;
                }
                :disabled {
                    color: #949497;
                    background-color: #F1F1F4;
                    border: #949497 1px solid;
                }
            "#
            ),
            Variant::Tertialy => style!(
                r#"
                color: #003EE5;
                border: #F1F1F4 1px solid;
                background-color: #F1F1F4;
                text-decoration: underline;
                text-decoration-color: #003EE5;
                :active {
                    border: #D18E0F 1px solid;
                }
                :disabled {
                    color: #949497;
                    border: #F1F1F4 1px solid;
                    text-decoration-color: #949497;
                }
            "#
            ),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub size: Option<Size>,
    pub variant: Option<Variant>,
    pub disabled: Option<bool>,
    pub on_click: Callback<MouseEvent>,
}

#[styled_component(AppButton)]
pub fn app_button(
    Props {
        name,
        size,
        variant,
        disabled,
        on_click,
    }: &Props,
) -> Html {
    let size_style = match size {
        Some(size) => size,
        None => &Size::Small,
    }
    .style()
    .expect("Failed to mount style");

    let variant_style = match variant {
        Some(variant) => variant,
        None => &Variant::Primary,
    }
    .style()
    .expect("Failed to mount style");

    let _disabled = match disabled {
        Some(disabled) => disabled,
        None => &false,
    };

    let base_style = style!(
        r#"
        border-radius: 4px;
        margin: 0;
        outline: none;
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        cursor: pointer;
        :disabled {
            cursor: not-allowed;
        }
    "#
    )
    .expect("Failed to mount style");

    html! {
        <input
            class={vec![base_style, size_style, variant_style]}
            disabled={*_disabled}
            type="button"
            value={name.clone()}
            onclick={on_click.clone()}
        />
    }
}
