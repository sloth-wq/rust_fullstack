use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_box::{AppBox, Padding};
use apps_frontend_package_ui::stack::{Spacing, VStack};
use apps_frontend_package_ui::{
    app_radio_group::{AppRadioButtonGroup, AppRadioCardGroup, Item},
    app_typography::{AppTypography, Color, Tag},
};
use gloo::console::console_dbg;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::{function_component, html, Callback, Html};
use yew::use_state;

#[function_component(AppRadioGroupPage)]
pub fn app_radio_group_page() -> Html {
    let item_list_1 = vec![
        Item {
            label: "Select 1".to_string(),
            value: "group1-1".to_string(),
            checked: true,
            disabled: false,
        },
        Item {
            label: "Select 2".to_string(),
            value: "group1-2".to_string(),
            checked: false,
            disabled: false,
        },
        Item {
            label: "Select 3".to_string(),
            value: "group1-3".to_string(),
            checked: false,
            disabled: true,
        },
        Item {
            label: "Select 4".to_string(),
            value: "group1-4".to_string(),
            checked: false,
            disabled: true,
        },
    ];

    let item_list_2 = vec![
        Item {
            label: "Select 1".to_string(),
            value: "group2-1".to_string(),
            checked: true,
            disabled: false,
        },
        Item {
            label: "Select 2".to_string(),
            value: "group2-2".to_string(),
            checked: false,
            disabled: false,
        },
        Item {
            label: "Select 3".to_string(),
            value: "group2-3".to_string(),
            checked: false,
            disabled: true,
        },
        Item {
            label: "Select 4".to_string(),
            value: "group2-4".to_string(),
            checked: false,
            disabled: true,
        },
    ];

    let state1 = use_state(|| item_list_1);
    let callback1 = {
        let _state = state1.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            let items = _state
                .iter()
                .cloned()
                .map(|item| Item {
                    checked: item.value.eq(&value),
                    ..item
                })
                .collect::<Vec<Item>>();

            console_dbg!(&items[0], &items[1]);
            _state.set(items);
        })
    };

    let state2 = use_state(|| item_list_2);
    let callback2 = {
        let _state = state2.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            let items = _state
                .iter()
                .cloned()
                .map(|item| Item {
                    checked: item.value.eq(&value),
                    ..item
                })
                .collect::<Vec<Item>>();

            console_dbg!(&items[0], &items[1]);
            _state.set(items);
        })
    };

    html!(
        <>
            <DefaultLayout>
                <VStack spacing={Spacing::Five}>
                    <AppBox>
                        <AppTypography value="AppRadioGroup" tag={Tag::H1} />
                        <AppTypography value="AppRadioGroup module is apps_frontend_package_ui::app_radio_group::*" tag={Tag::P} color={Color::Gray} />
                        <AppTypography value="This page module is apps_frontend_design_sys::app_radio_group_page::*" tag={Tag::P} color={Color::Gray} />
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
                                <AppTypography value="AppRadioButtonGroup Default" tag={Tag::H2} />
                                <AppRadioButtonGroup
                                    name={"group1-1"}
                                    item_list={state1.iter().cloned().collect::<Vec<Item>>()}
                                    on_change={callback1.clone()}
                                />
                            </AppBox>
                        </VStack>
                        <VStack spacing={Spacing::Two}>
                            <AppBox>
                                <AppTypography value="AppRadioButtonGroup Error" tag={Tag::H2} />
                                <AppRadioButtonGroup
                                    name={"group1-2"}
                                    item_list={state1.iter().cloned().collect::<Vec<Item>>()}
                                    is_error={true}
                                    on_change={callback1.clone()}
                                />
                            </AppBox>
                        </VStack>
                    </AppBox>

                    <AppBox pr={Padding::Two} pl={Padding::Two}>
                        <VStack spacing={Spacing::Two}>
                            <AppBox>
                                <AppTypography value="AppRadioCardGroup" tag={Tag::H2} />
                                <AppRadioCardGroup
                                    name={"group2-1"}
                                    item_list={state2.iter().cloned().collect::<Vec<Item>>()}
                                    on_change={callback2.clone()}
                                />
                            </AppBox>
                        </VStack>
                        <VStack spacing={Spacing::Two}>
                            <AppBox>
                                <AppTypography value="AppRadioButtonGroup Error" tag={Tag::H2} />
                                <AppRadioCardGroup
                                    name={"group2-2"}
                                    item_list={state2.iter().cloned().collect::<Vec<Item>>()}
                                    is_error={true}
                                    on_change={callback2.clone()}
                                />
                            </AppBox>
                        </VStack>
                    </AppBox>
                </VStack>
            </DefaultLayout>
        </>
    )
}
