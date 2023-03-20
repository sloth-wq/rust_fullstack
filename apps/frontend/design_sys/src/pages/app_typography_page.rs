use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::{AppBox, BackgroundColor, Padding};
use apps_frontend_package_ui::app_typography::{AppTypography, Color, Tag};
use apps_frontend_package_ui::stack::{HStack, JustifyContent, Spacing, VStack};
use yew::prelude::{function_component, html, Html};

#[function_component(AppTypographyPage)]
pub fn app_typography_page() -> Html {
    html!(
        <DefaultLayout>
            <VStack spacing={Spacing::Five}>
                <AppBox>
                    <AppTypography value="AppTypography" tag={Tag::H1} />
                    <AppTypography value="AppTypography module is apps_frontend_package_ui::app_typography::*" tag={Tag::P} color={Color::Gray} />
                    <AppTypography value="This page module is apps_frontend_design_sys::app_typography::*" tag={Tag::P} color={Color::Gray} />
                </AppBox>

                <AppBox>
                    <AppTypography value="Description" tag={Tag::H2} />
                    <AppTypography value="None" tag={Tag::P} color={Color::Gray} />
                </AppBox>

                <AppBox>
                    <AppTypography value="Usage" tag={Tag::H2} />
                </AppBox>

                <AppBox pr={Padding::Two} pl={Padding::Two}>
                    <HStack spacing={Spacing::Eight} justify_content={JustifyContent::FlexStart}>
                        <AppBox>
                            <AppTypography value="Black" tag={Tag::H3} />

                            <AppTypography value="h1" tag={Tag::H1} />
                            <AppTypography value="h2" tag={Tag::H2} />
                            <AppTypography value="h3" tag={Tag::H3} />
                            <AppTypography value="h4" tag={Tag::H4} />
                            <AppTypography value="h5" tag={Tag::H5} />
                            <AppTypography value="h6" tag={Tag::H6} />
                            <AppTypography value="p" tag={Tag::P} />
                            <AppTypography value="span" tag={Tag::Span} />
                        </AppBox>

                        <AppBox background_color={BackgroundColor::Gray}>
                            <AppTypography value="White" tag={Tag::H3} color={Color::White} />

                            <AppTypography value="h1" tag={Tag::H1} color={Color::White} />
                            <AppTypography value="h2" tag={Tag::H2} color={Color::White} />
                            <AppTypography value="h3" tag={Tag::H3} color={Color::White} />
                            <AppTypography value="h4" tag={Tag::H4} color={Color::White} />
                            <AppTypography value="h5" tag={Tag::H5} color={Color::White} />
                            <AppTypography value="h6" tag={Tag::H6} color={Color::White} />
                            <AppTypography value="p" tag={Tag::P} color={Color::White} />
                            <AppTypography value="span" tag={Tag::Span} color={Color::White} />
                        </AppBox>

                        <AppBox>
                            <AppTypography value="Red" tag={Tag::H3} />

                            <AppTypography value="h1" tag={Tag::H1} color={Color::Red} />
                            <AppTypography value="h2" tag={Tag::H2} color={Color::Red} />
                            <AppTypography value="h3" tag={Tag::H3} color={Color::Red} />
                            <AppTypography value="h4" tag={Tag::H4} color={Color::Red} />
                            <AppTypography value="h5" tag={Tag::H5} color={Color::Red} />
                            <AppTypography value="h6" tag={Tag::H6} color={Color::Red} />
                            <AppTypography value="p" tag={Tag::P} color={Color::Red} />
                            <AppTypography value="span" tag={Tag::Span} color={Color::Red} />
                        </AppBox>

                        <AppBox>
                            <AppTypography value="Gray" tag={Tag::H3} />

                            <AppTypography value="h1" tag={Tag::H1} color={Color::Gray} />
                            <AppTypography value="h2" tag={Tag::H2} color={Color::Gray} />
                            <AppTypography value="h3" tag={Tag::H3} color={Color::Gray} />
                            <AppTypography value="h4" tag={Tag::H4} color={Color::Gray} />
                            <AppTypography value="h5" tag={Tag::H5} color={Color::Gray} />
                            <AppTypography value="h6" tag={Tag::H6} color={Color::Gray} />
                            <AppTypography value="p" tag={Tag::P} color={Color::Gray} />
                            <AppTypography value="span" tag={Tag::Span} color={Color::Gray} />
                        </AppBox>
                    </HStack>
                </AppBox>
            </VStack>
        </DefaultLayout>
    )
}
