use stylist::style;
use stylist::Style;
use yew::prelude::{html, Html, Properties};
use yew::{function_component, Children};

#[derive(PartialEq)]
pub enum Spacing {
    None,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Spacing {
    fn style(&self) -> Result<Style, stylist::Error> {
        match self {
            Spacing::None => style!(r#""#),
            Spacing::One => style!(r#"gap: 8px;"#),
            Spacing::Two => style!(r#"gap: 16px;"#),
            Spacing::Three => style!(r#"gap: 24px;"#),
            Spacing::Four => style!(r#"gap: 32px;"#),
            Spacing::Five => style!(r#"gap: 40px;"#),
            Spacing::Six => style!(r#"gap: 48px;"#),
            Spacing::Seven => style!(r#"gap: 56px;"#),
            Spacing::Eight => style!(r#"gap: 64px;"#),
        }
    }
}

#[derive(PartialEq)]
pub enum JustifyContent {
    Center,
    SpaceBetween,
    FlexStart,
    FlexEnd,
}

impl JustifyContent {
    fn style(&self) -> Result<Style, stylist::Error> {
        match self {
            JustifyContent::Center => style!(r#"justify-content: center;"#),
            JustifyContent::SpaceBetween => style!(r#"justify-content: space-between;"#),
            JustifyContent::FlexStart => style!(r#"justify-content: flex-start;"#),
            JustifyContent::FlexEnd => style!(r#"justify-content: flex-end;"#),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub spacing: Option<Spacing>,
    pub justify_content: Option<JustifyContent>,
    pub children: Children,
}

// 横並び.
#[function_component(HStack)]
pub fn h_stack(
    Props {
        spacing,
        justify_content,
        children,
    }: &Props,
) -> Html {
    let container_style = style!(r#"display: flex;"#).expect("Failed to mount style");
    let spacing_style = match spacing {
        Some(v) => v,
        None => &Spacing::None,
    }
    .style()
    .expect("Failed to mount style");
    let justify_content_style = match justify_content {
        Some(jc) => jc,
        None => &JustifyContent::Center,
    }
    .style()
    .expect("Failed to mount style");

    html! {
        <>
            <div class={vec!(container_style, spacing_style, justify_content_style)}>
                { for children.iter() }
            </div>
        </>
    }
}

// 縦並び.
#[function_component(VStack)]
pub fn v_stack(
    Props {
        spacing,
        children,
        justify_content,
    }: &Props,
) -> Html {
    let container_style = style!(
        r#"
        display: flex;
        flex-direction: column;
    "#
    )
    .expect("Failed to mount style");
    let spacing_style = match spacing {
        Some(v) => v,
        None => &Spacing::None,
    }
    .style()
    .expect("Failed to mount style");
    let justify_content_style = match justify_content {
        Some(jc) => jc,
        None => &JustifyContent::Center,
    }
    .style()
    .expect("Failed to mount style");

    html! {
        <>
            <div class={vec!(container_style, spacing_style, justify_content_style)}>
                { for children.iter() }
            </div>
        </>
    }
}
