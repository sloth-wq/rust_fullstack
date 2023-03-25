use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::{AppBox, Padding};
use apps_frontend_package_ui::stack::{Spacing, VStack};
use apps_frontend_package_ui::{
    app_checkbox::AppCheckbox,
    app_typography::{AppTypography, Color, Tag},
};
use gloo::console::console_dbg;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::{function_component, html, Callback, Html};
use yew::use_state;

#[function_component(AppCheckboxPage)]
pub fn app_checkbox_page() -> Html {
    let checked = use_state(|| false);
    let callback = {
        let _checked = checked.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();

            console_dbg!(&value);
            _checked.set(value);
        })
    };

    html!(
        <>
            <DefaultLayout>
                <VStack spacing={Spacing::Five}>
                    <AppBox>
                        <AppTypography value="AppCheckbox" tag={Tag::H1} />
                        <AppTypography value="AppCheckbox module is apps_frontend_package_ui::app_checkbox::*" tag={Tag::P} color={Color::Gray} />
                        <AppTypography value="This page module is apps_frontend_design_sys::app_checkbox_page::*" tag={Tag::P} color={Color::Gray} />
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
                                <AppTypography value="Default" tag={Tag::H2} />
                                <AppCheckbox label="Select 1" value="checkbox1" checked={*checked} on_change={callback} />
                                <AppCheckbox label="Select 2" value="checkbox2" checked={false} />
                            </AppBox>

                            <AppBox>
                                <AppTypography value="Disabled" tag={Tag::H2} />
                                <AppCheckbox label="Select 3" value="checkbox3" checked={true} disabled={true} />
                                <AppCheckbox label="Select 4" value="checkbox4" checked={false} disabled={true} />
                            </AppBox>
                        </VStack>
                    </AppBox>
                </VStack>
            </DefaultLayout>
        </>
    )
}
