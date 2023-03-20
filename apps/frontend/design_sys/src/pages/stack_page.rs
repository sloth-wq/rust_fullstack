use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::{AppBox, Padding};
use apps_frontend_package_ui::app_typography::{AppTypography, Color, Tag};
use apps_frontend_package_ui::stack::{HStack, JustifyContent, Spacing, VStack};
use stylist::style;
use yew::prelude::{function_component, html, Html};

#[function_component(StackPage)]
pub fn stack_page() -> Html {
    let square_style = style!(
        r#"
            height: 50px;
            width: 50px;
            background-color: #003EE5;
    "#
    )
    .unwrap();

    html!(
        <>
            <DefaultLayout>
                <VStack spacing={Spacing::Five}>
                    <AppBox>
                        <AppTypography value="Stack" tag={Tag::H1} />
                        <AppTypography value="Stack module is apps_frontend_package_ui::stack::*" tag={Tag::P} color={Color::Gray} />
                        <AppTypography value="This page module is apps_frontend_design_sys::stack::*" tag={Tag::P} color={Color::Gray} />
                    </AppBox>
                    <VStack spacing={Spacing::Three}>
                        <AppBox>
                            <AppTypography value="Description" tag={Tag::H2} />
                            <AppTypography value="HStack is horizontal, VStack is vertical" tag={Tag::P} color={Color::Gray} />
                        </AppBox>

                        <AppBox>
                            <AppTypography value="Usage" tag={Tag::H2} />
                        </AppBox>

                        <AppBox pr={Padding::Two} pl={Padding::Two}>
                            <VStack spacing={Spacing::Two}>
                                <VStack spacing={Spacing::One}>
                                    <AppBox>
                                        <AppTypography value="Spacing" tag={Tag::H3} />
                                        <AppTypography value="Spacing = One * 8px" tag={Tag::P} color={Color::Gray} />
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="One" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::One} justify_content={JustifyContent::FlexStart}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="Two" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::Two} justify_content={JustifyContent::FlexStart}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="Three" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexStart}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="Four" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::Four} justify_content={JustifyContent::FlexStart}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>
                                </VStack>

                                <VStack spacing={Spacing::One}>
                                    <AppBox>
                                        <AppTypography value="Justify Content" tag={Tag::H3} />
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="Center" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::One} justify_content={JustifyContent::Center}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="FlexStart" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::One} justify_content={JustifyContent::FlexStart}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="FlexEnd" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::One} justify_content={JustifyContent::FlexEnd}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>

                                    <AppBox>
                                        <AppTypography value="SpaceBetween" tag={Tag::P} color={Color::Gray} />
                                        <HStack spacing={Spacing::One} justify_content={JustifyContent::SpaceBetween}>
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                            <div class={square_style.clone()} />
                                        </HStack>
                                    </AppBox>
                                </VStack>
                            </VStack>
                        </AppBox>
                    </VStack>
                </VStack>
            </DefaultLayout>
        </>
    )
}
