use crate::layout::layout::DefaultLayout;
use apps_frontend_package_ui::app_button::{AppButton, Size, Variant};
use gloo::dialogs::alert;
use stylist::{style, Style};
use yew::prelude::{function_component, html, Html};

#[function_component(AppButtonPage)]
pub fn app_button_page() -> Html {
    html!(
        <DefaultLayout>
            <AppButton name={"Large"} size={Size::Large} on_click={|_| {()}}></AppButton>
            <AppButton name={"Medium"} size={Size::Medium} on_click={|_| {()}}></AppButton>
            <AppButton name={"Small"} size={Size::Small} on_click={|_| {()}}></AppButton>

            <AppButton name={"Primary"} variant={Variant::Primary} on_click={|_| {()}}></AppButton>
            <AppButton name={"Secondary"} variant={Variant::Secondary} on_click={|_| {()}}></AppButton>
            <AppButton name={"Tertialy"} variant={Variant::Tertialy} on_click={|_| {()}}></AppButton>

            <AppButton name={"Primary"} variant={Variant::Primary} disabled={true} on_click={|_| {()}}></AppButton>
            <AppButton name={"Secondary"} variant={Variant::Secondary} disabled={true} on_click={|_| {()}}></AppButton>
            <AppButton name={"Tertialy"} variant={Variant::Tertialy} disabled={true} on_click={|_| {()}}></AppButton>

            <AppButton name={"Click Me"} variant={Variant::Primary} on_click={|_| {alert("Clicked")}}></AppButton>
        </DefaultLayout>
    )
}
