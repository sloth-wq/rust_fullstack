use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_typography::{AppTypography, Color, Tag};
use yew::prelude::{function_component, html, Html};

#[function_component(AppTypographyPage)]
pub fn app_typography_page() -> Html {
    html!(
        <DefaultLayout>
            <AppTypography value="h1" tag={Tag::H1} />
            <AppTypography value="h2" tag={Tag::H2} />
            <AppTypography value="h3" tag={Tag::H3} />
            <AppTypography value="h4" tag={Tag::H4} />
            <AppTypography value="h5" tag={Tag::H5} />
            <AppTypography value="h6" tag={Tag::H6} />
            <AppTypography value="p" tag={Tag::P} />
            <AppTypography value="span" tag={Tag::Span} />

            <AppTypography value="h1" tag={Tag::H1} color={Color::White} />
            <AppTypography value="h2" tag={Tag::H2} color={Color::White} />
            <AppTypography value="h3" tag={Tag::H3} color={Color::White} />
            <AppTypography value="h4" tag={Tag::H4} color={Color::White} />
            <AppTypography value="h5" tag={Tag::H5} color={Color::White} />
            <AppTypography value="h6" tag={Tag::H6} color={Color::White} />
            <AppTypography value="p" tag={Tag::P} color={Color::White} />
            <AppTypography value="span" tag={Tag::Span} color={Color::White} />

            <AppTypography value="h1" tag={Tag::H1} color={Color::Red} />
            <AppTypography value="h2" tag={Tag::H2} color={Color::Red} />
            <AppTypography value="h3" tag={Tag::H3} color={Color::Red} />
            <AppTypography value="h4" tag={Tag::H4} color={Color::Red} />
            <AppTypography value="h5" tag={Tag::H5} color={Color::Red} />
            <AppTypography value="h6" tag={Tag::H6} color={Color::Red} />
            <AppTypography value="p" tag={Tag::P} color={Color::Red} />
            <AppTypography value="span" tag={Tag::Span} color={Color::Red} />

            <AppTypography value="h1" tag={Tag::H1} color={Color::Gray} />
            <AppTypography value="h2" tag={Tag::H2} color={Color::Gray} />
            <AppTypography value="h3" tag={Tag::H3} color={Color::Gray} />
            <AppTypography value="h4" tag={Tag::H4} color={Color::Gray} />
            <AppTypography value="h5" tag={Tag::H5} color={Color::Gray} />
            <AppTypography value="h6" tag={Tag::H6} color={Color::Gray} />
            <AppTypography value="p" tag={Tag::P} color={Color::Gray} />
            <AppTypography value="span" tag={Tag::Span} color={Color::Gray} />
        </DefaultLayout>
    )
}
