use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::{AppBox, Padding};
use apps_frontend_package_ui::stack::{Spacing, VStack};
use apps_frontend_package_ui::{
    app_textarea::AppTextarea,
    app_typography::{AppTypography, Color, Tag},
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::{function_component, html, Callback, Html};
use yew::{use_state, InputEvent};

#[function_component(AppTextareaPage)]
pub fn app_textarea_page() -> Html {
    let handler1 = use_state(|| "".to_string());
    let callback1 = {
        let _handler = handler1.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            _handler.set(value);
        })
    };

    let handler2 = use_state(|| "".to_string());
    let callback2 = {
        let _handler = handler2.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            _handler.set(value)
        })
    };

    let handler3 = use_state(|| "".to_string());
    let callback3 = {
        let _handler = handler3.clone();

        Callback::from(move |event: InputEvent| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            _handler.set(value)
        })
    };

    html!(
        <>
            <DefaultLayout>
                <VStack spacing={Spacing::Five}>
                    <AppBox>
                        <AppTypography value="AppTextarea" tag={Tag::H1} />
                        <AppTypography value="AppTextarea module is apps_frontend_package_ui::app_textarea::*" tag={Tag::P} color={Color::Gray} />
                        <AppTypography value="This page module is apps_frontend_design_sys::app_textarea_page::*" tag={Tag::P} color={Color::Gray} />
                    </AppBox>

                    <AppBox>
                        <AppTypography value="Description" tag={Tag::H2} />
                        <AppTypography value="None" tag={Tag::P} color={Color::Gray} />
                    </AppBox>

                    <AppBox>
                        <AppTypography value="Usage" tag={Tag::H2} />
                    </AppBox>

                    <AppBox pr={Padding::Two} pl={Padding::Two}>
                        <VStack spacing={Spacing::Two}>
                            <AppBox>
                                <AppTypography value="Default" tag={Tag::H3} />
                                <AppBox>
                                    <AppTextarea value={String::from(&*handler1)} on_input={callback1.clone()} />
                                </AppBox>
                                <AppBox>
                                    {"Input Text: "}{&*handler1}
                                </AppBox>
                            </AppBox>

                            <AppBox>
                                <AppTypography value="Disabled" tag={Tag::H3} />
                                <AppBox>
                                    <AppTextarea value={"Disabled".to_string()} disabled={true} on_input={callback1.clone()} />
                                </AppBox>
                            </AppBox>

                            <AppBox>
                                <AppTypography value="Placeholder" tag={Tag::H3} />
                                <AppBox>
                                    <AppTextarea value={String::from(&*handler2)} placeholder={"Placeholder".to_string()} on_input={callback2.clone()} />
                                </AppBox>
                                <AppBox>
                                    {"Input Text: "}{&*handler2}
                                </AppBox>
                            </AppBox>

                            <AppBox>
                                <AppTypography value="Error" tag={Tag::H3} />
                                <AppBox>
                                    <AppTextarea value={String::from(&*handler3)} is_error={true} on_input={callback3.clone()} />
                                </AppBox>
                                    <AppBox>
                                        {"Input Text: "}{&*handler3}
                                    </AppBox>
                            </AppBox>
                        </VStack>
                    </AppBox>
                </VStack>
            </DefaultLayout>
        </>
    )
}
