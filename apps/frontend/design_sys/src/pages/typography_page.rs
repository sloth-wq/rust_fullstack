use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::typography::{Color, Tag, Typography};
use gloo::dialogs::alert;
use stylist::{style, Style};
use yew::prelude::{function_component, html, Html};

#[function_component(TypographyPage)]
pub fn typography_page() -> Html {
    html!(
        <DefaultLayout>
            <Typography value="h1" tag={Tag::H1} />
            <Typography value="h2" tag={Tag::H2} />
            <Typography value="h3" tag={Tag::H3} />
            <Typography value="h4" tag={Tag::H4} />
            <Typography value="h5" tag={Tag::H5} />
            <Typography value="h6" tag={Tag::H6} />
            <Typography value="p" tag={Tag::P} />
            <Typography value="span" tag={Tag::Span} />

            <Typography value="h1" tag={Tag::H1} color={Color::White} />
            <Typography value="h2" tag={Tag::H2} color={Color::White} />
            <Typography value="h3" tag={Tag::H3} color={Color::White} />
            <Typography value="h4" tag={Tag::H4} color={Color::White} />
            <Typography value="h5" tag={Tag::H5} color={Color::White} />
            <Typography value="h6" tag={Tag::H6} color={Color::White} />
            <Typography value="p" tag={Tag::P} color={Color::White} />
            <Typography value="span" tag={Tag::Span} color={Color::White} />

            <Typography value="h1" tag={Tag::H1} color={Color::Red} />
            <Typography value="h2" tag={Tag::H2} color={Color::Red} />
            <Typography value="h3" tag={Tag::H3} color={Color::Red} />
            <Typography value="h4" tag={Tag::H4} color={Color::Red} />
            <Typography value="h5" tag={Tag::H5} color={Color::Red} />
            <Typography value="h6" tag={Tag::H6} color={Color::Red} />
            <Typography value="p" tag={Tag::P} color={Color::Red} />
            <Typography value="span" tag={Tag::Span} color={Color::Red} />

            <Typography value="h1" tag={Tag::H1} color={Color::Gray} />
            <Typography value="h2" tag={Tag::H2} color={Color::Gray} />
            <Typography value="h3" tag={Tag::H3} color={Color::Gray} />
            <Typography value="h4" tag={Tag::H4} color={Color::Gray} />
            <Typography value="h5" tag={Tag::H5} color={Color::Gray} />
            <Typography value="h6" tag={Tag::H6} color={Color::Gray} />
            <Typography value="p" tag={Tag::P} color={Color::Gray} />
            <Typography value="span" tag={Tag::Span} color={Color::Gray} />
        </DefaultLayout>
    )
}
