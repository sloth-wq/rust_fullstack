use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::stack::{HStack, JustifyContent, Spacing, VStack};
use yew::prelude::{function_component, html, Html};

#[function_component(StackPage)]
pub fn stack_page() -> Html {
    html!(
        <>
            <DefaultLayout>
                {"h stack"}
                {"center"}
                <HStack spacing={Spacing::One}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Two}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Three}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Four}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                {"space between"}
                <HStack spacing={Spacing::One} justify_content={JustifyContent::SpaceBetween}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Two} justify_content={JustifyContent::SpaceBetween}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Three} justify_content={JustifyContent::SpaceBetween}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Four} justify_content={JustifyContent::SpaceBetween}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                {"flex start"}
                <HStack spacing={Spacing::One} justify_content={JustifyContent::FlexStart}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Two} justify_content={JustifyContent::FlexStart}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexStart}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Four} justify_content={JustifyContent::FlexStart}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                {"flex end"}
                <HStack spacing={Spacing::One} justify_content={JustifyContent::FlexEnd}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Two} justify_content={JustifyContent::FlexEnd}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Three} justify_content={JustifyContent::FlexEnd}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                <HStack spacing={Spacing::Four} justify_content={JustifyContent::FlexEnd}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </HStack>

                {"v stack"}
                {"center"}
                <VStack spacing={Spacing::One}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </VStack>

                <VStack spacing={Spacing::Two}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </VStack>

                <VStack spacing={Spacing::Three}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </VStack>

                <VStack spacing={Spacing::Four}>
                    <div>{"1"}</div>
                    <div>{"2"}</div>
                    <div>{"3"}</div>
                </VStack>
            </DefaultLayout>
        </>
    )
}
