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
    #[prop_or(BackgroundColor::White)]
    pub background_color: BackgroundColor,
    #[prop_or(Padding::None)]
    pub pt: Padding,
    #[prop_or(Padding::None)]
    pub pb: Padding,
    #[prop_or(Padding::None)]
    pub pr: Padding,
    #[prop_or(Padding::None)]
    pub pl: Padding,
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
    let background_color_style = background_color.style().unwrap();

    let pt_style = pt.pt_style().unwrap();
    let pb_style = pb.pb_style().unwrap();
    let pr_style = pr.pr_style().unwrap();
    let pl_style = pl.pl_style().unwrap();

    html! {
        <div class={vec!(background_color_style, pt_style, pb_style, pr_style, pl_style)}>
            { for children.iter() }
        </div>
    }
}
