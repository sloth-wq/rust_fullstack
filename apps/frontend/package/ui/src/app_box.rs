use std::vec;

use stylist::{style, Style};
use yew::prelude::{html, Html, Properties};
use yew::{function_component, Children};

#[derive(PartialEq)]
pub enum BackgroundColor {
    White,
    Gray,
}

impl BackgroundColor {
    fn style(&self) -> Result<Style, stylist::Error> {
        match self {
            BackgroundColor::White => style!(
                r#"
                    background-color: white;
                "#
            ),
            BackgroundColor::Gray => style!(
                r#"
                    background-color: #949497;
                "#
            ),
        }
    }
}

#[derive(PartialEq)]
pub enum Padding {
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

impl Padding {
    fn pt_style(&self) -> Result<Style, stylist::Error> {
        match self {
            Padding::None => style!(r#"padding-top: 0px;"#),
            Padding::One => style!(r#"padding-top: 8px;"#),
            Padding::Two => style!(r#"padding-top: 16px;"#),
            Padding::Three => style!(r#"padding-top: 24px;"#),
            Padding::Four => style!(r#"padding-top: 32px;"#),
            Padding::Five => style!(r#"padding-top: 40px;"#),
            Padding::Six => style!(r#"padding-top: 48px;"#),
            Padding::Seven => style!(r#"padding-top: 56px;"#),
            Padding::Eight => style!(r#"padding-top: 64px;"#),
        }
    }

    fn pb_style(&self) -> Result<Style, stylist::Error> {
        match self {
            Padding::None => style!(r#"padding-top: 0px;"#),
            Padding::One => style!(r#"padding-botton: 8px;"#),
            Padding::Two => style!(r#"padding-botton: 16px;"#),
            Padding::Three => style!(r#"padding-botton: 24px;"#),
            Padding::Four => style!(r#"padding-botton: 32px;"#),
            Padding::Five => style!(r#"padding-botton: 40px;"#),
            Padding::Six => style!(r#"padding-botton: 48px;"#),
            Padding::Seven => style!(r#"padding-botton: 56px;"#),
            Padding::Eight => style!(r#"padding-botton: 64px;"#),
        }
    }

    fn pr_style(&self) -> Result<Style, stylist::Error> {
        match self {
            Padding::None => style!(r#"padding-top: 0px;"#),
            Padding::One => style!(r#"padding-right: 8px;"#),
            Padding::Two => style!(r#"padding-right: 16px;"#),
            Padding::Three => style!(r#"padding-right: 24px;"#),
            Padding::Four => style!(r#"padding-right: 32px;"#),
            Padding::Five => style!(r#"padding-right: 40px;"#),
            Padding::Six => style!(r#"padding-right: 48px;"#),
            Padding::Seven => style!(r#"padding-right: 56px;"#),
            Padding::Eight => style!(r#"padding-right: 64px;"#),
        }
    }

    fn pl_style(&self) -> Result<Style, stylist::Error> {
        match self {
            Padding::None => style!(r#"padding-top: 0px;"#),
            Padding::One => style!(r#"padding-left: 8px;"#),
            Padding::Two => style!(r#"padding-left: 16px;"#),
            Padding::Three => style!(r#"padding-left: 24px;"#),
            Padding::Four => style!(r#"padding-left: 32px;"#),
            Padding::Five => style!(r#"padding-left: 40px;"#),
            Padding::Six => style!(r#"padding-left: 48px;"#),
            Padding::Seven => style!(r#"padding-left: 56px;"#),
            Padding::Eight => style!(r#"padding-left: 64px;"#),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub background_color: Option<BackgroundColor>,
    pub pt: Option<Padding>,
    pub pb: Option<Padding>,
    pub pr: Option<Padding>,
    pub pl: Option<Padding>,
    pub children: Children,
}

#[function_component(AppBox)]
pub fn app_box(
    Props {
        background_color,
        pt,
        pb,
        pr,
        pl,
        children,
    }: &Props,
) -> Html {
    let background_color_style = match background_color {
        Some(bc) => bc,
        None => &BackgroundColor::White,
    }
    .style()
    .expect("Failed to mount style");

    let pt_style = match pt {
        Some(v) => v,
        None => &Padding::None,
    }
    .pt_style()
    .expect("Failed to mount style");

    let pb_style = match pb {
        Some(v) => v,
        None => &Padding::None,
    }
    .pb_style()
    .expect("Failed to mount style");

    let pr_style = match pr {
        Some(v) => v,
        None => &Padding::None,
    }
    .pr_style()
    .expect("Failed to mount style");

    let pl_style = match pl {
        Some(v) => v,
        None => &Padding::None,
    }
    .pl_style()
    .expect("Failed to mount style");

    html! {
        <>
            <div class={vec!(background_color_style, pt_style, pb_style, pr_style, pl_style)}>
                { for children.iter() }
            </div>
        </>
    }
}
