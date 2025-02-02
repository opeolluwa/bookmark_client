use std::str::FromStr;

use leptos::either::Either;
use leptos::prelude::{signal, Get, Set};
use leptos::{component, view, IntoView};
use thaw::ConfigProvider;

use bookmark_desktop_app::app::DesktopApplication;
use bookmark_mobile_app::app::MobileApplication;
use tauri_wasm_bindgen::plugins::os::get_device_operating_system;

use crate::platform::Platform;

#[component]
pub fn App() -> impl IntoView {
    let (device_operating_system, set_device_operating_system) = signal(String::new());
    let operating_system = get_device_operating_system().as_string();
    set_device_operating_system.set(operating_system.unwrap());

    let device_platform = Platform::from_str(&device_operating_system.get()).unwrap_or_default();

    view! {
        <ConfigProvider>
            {match device_platform {
                Platform::Android | Platform::Ios => Either::Left(view! { <MobileApplication /> }),
                _ => Either::Right(view! { <DesktopApplication /> }),
            }}
        </ConfigProvider>
    }
}
