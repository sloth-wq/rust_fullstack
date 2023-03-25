use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::AppBox;
use apps_frontend_package_ui::app_typography::{AppTypography, Color, Tag};
use apps_frontend_package_ui::stack::{Spacing, VStack};
use yew::prelude::{function_component, html, Html};

#[function_component(AppBoxPage)]
pub fn app_box_page() -> Html {
    html!(
        <DefaultLayout>
            <VStack spacing={Spacing::Five}>
                <AppBox>
                    <AppTypography value="AppBox" tag={Tag::H1} />
                    <AppTypography value="AppBox module is apps_frontend_package_ui::app_button::*" tag={Tag::P} color={Color::Gray} />
                    <AppTypography value="This page module is apps_frontend_design_sys::app_box::*" tag={Tag::P} color={Color::Gray} />
                </AppBox>

                <AppBox>
                    <AppTypography value="Description" tag={Tag::H2} />
                    <AppTypography value="AppBox = Html div tag." tag={Tag::P} color={Color::Gray} />
                </AppBox>

                <AppBox>
                    <AppTypography value="Usage" tag={Tag::H2} />
                    <AppTypography value="Specifiable props are background-color and padding." tag={Tag::P} color={Color::Gray} />
                </AppBox>
            </VStack>
        </DefaultLayout>
    )
}
