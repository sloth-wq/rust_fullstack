use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::{AppBox, Padding};
use apps_frontend_package_ui::app_button::{AppButton, Size, Variant};
use apps_frontend_package_ui::app_typography::{AppTypography, Color, Tag};
use apps_frontend_package_ui::stack::{HStack, JustifyContent, Spacing, VStack};
use gloo::dialogs::alert;
use yew::prelude::{function_component, html, Html};

#[function_component(AppButtonPage)]
pub fn app_button_page() -> Html {
    html!(
        <DefaultLayout>
            <VStack spacing={Spacing::Five}>
                <AppBox>
                    <AppTypography value="AppButton" tag={Tag::H1} />
                    <AppTypography value="AppButton module is apps_frontend_package_ui::app_button::*" tag={Tag::P} color={Color::Gray} />
                    <AppTypography value="This page module is apps_frontend_design_sys::app_button_page::*" tag={Tag::P} color={Color::Gray} />
                </AppBox>

                <AppBox>
                    <AppTypography value="Usage" tag={Tag::H2} />
                </AppBox>

                <AppBox pr={Padding::Two} pl={Padding::Two}>
                    <VStack spacing={Spacing::Three}>
                        <AppBox>
                            <AppTypography value="Size" tag={Tag::H3} />
                            <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexStart}>
                                <AppButton name={"Large"} size={Size::Large} on_click={|_| {()}}></AppButton>
                                <AppButton name={"Medium"} size={Size::Medium} on_click={|_| {()}}></AppButton>
                                <AppButton name={"Small"} size={Size::Small} on_click={|_| {()}}></AppButton>
                            </HStack>
                        </AppBox>

                        <AppBox>
                            <AppTypography value="Variant" tag={Tag::H3} />
                            <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexStart}>
                                <AppButton name={"Primary"} variant={Variant::Primary} on_click={|_| {()}}></AppButton>
                                <AppButton name={"Secondary"} variant={Variant::Secondary} on_click={|_| {()}}></AppButton>
                                <AppButton name={"Tertialy"} variant={Variant::Tertialy} on_click={|_| {()}}></AppButton>
                            </HStack>
                        </AppBox>

                        <AppBox>
                            <AppTypography value="Disabled" tag={Tag::H3} />
                            <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexStart}>
                                <AppButton name={"Primary"} variant={Variant::Primary} disabled={true} on_click={|_| {()}}></AppButton>
                                <AppButton name={"Secondary"} variant={Variant::Secondary} disabled={true} on_click={|_| {()}}></AppButton>
                                <AppButton name={"Tertialy"} variant={Variant::Tertialy} disabled={true} on_click={|_| {()}}></AppButton>
                            </HStack>
                        </AppBox>

                        <AppBox>
                            <AppTypography value="Event" tag={Tag::H3} />
                            <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexStart}>
                                <AppButton name={"Click me"} variant={Variant::Primary} on_click={|_| {alert("Dispatched click event!")}}></AppButton>
                            </HStack>
                        </AppBox>
                    </VStack>
                </AppBox>
            </VStack>
        </DefaultLayout>
    )
}
