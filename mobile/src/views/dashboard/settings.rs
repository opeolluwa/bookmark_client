use bookmark_components::{
    icons::{
        bell_icon::BellIconOutline, language_icon::LanguageIcon,
        password_settings::ShieldIconOutline, user_icon::UserIconOutline,
    },
    settings::{Language, SettingsRoute, SettingsTab},
    typography::heading::PageHeading,
};
use leptos::{
    prelude::{ClassAttribute, CollectView, CustomAttribute, ElementChild},
    view,
};
use thaw::{Menu, MenuItem, MenuTrigger, MenuTriggerType};

use crate::layout::dashboard_layout::DashboardLayout;

#[leptos::component]
pub fn SettingsPage() -> impl leptos::IntoView {
    let language_select_component = LanguageComponent();
    let _account_settings_component = AccountSettingsComponent();
    let color_theme_swap_component = ColorThemeSwapComponent();

    view! {
        <DashboardLayout header_component=view! { <PageHeading text="settings" /> }>

            // <SettingsTab
            // title="Account Settings"
            // component=account_settings_component
            // class="mt-0 hidden"
            // />
            <SettingsTab
                title="Preferred language"
                component=language_select_component
                class="mt-0"
            />
            <SettingsTab title="Color theme" component=color_theme_swap_component />
        </DashboardLayout>
    }
}

#[leptos::component]
pub fn LanguageComponent() -> impl leptos::IntoView {
    let languages = Language::collect();
    let language_icon = LanguageIcon();
    let on_select = move |key: String| {
        leptos::logging::warn!("{}", key);
    };

    view! {
        <Menu on_select trigger_type=MenuTriggerType::Hover class="w-full">
            <MenuTrigger slot>
                <SettingsRoute icon=language_icon label="English" />
            </MenuTrigger>
            {languages
                .into_iter()
                .map(|(label, value)| {
                    view! {
                        <MenuItem value=value class="border-none capitalize w-24">
                            {label}
                        </MenuItem>
                    }
                })
                .collect_view()}
        </Menu>
    }
}

#[leptos::component]
pub fn AccountSettingsComponent() -> impl leptos::IntoView {
    let user_icon = UserIconOutline();
    let password_icon = ShieldIconOutline();
    let notification_icon = BellIconOutline();

    view! {
        <div class="flex flex-col gap-y-3">
            <SettingsRoute icon=user_icon label="Account Information" />
            <SettingsRoute icon=password_icon label="Password and Security" />
            <SettingsRoute icon=notification_icon label="Notification Preferences" />
        </div>
    }
}

#[leptos::component]
pub fn ColorThemeSwapComponent() -> impl leptos::IntoView {
    view! {
        <label class="swap swap-rotate">
            <input type="checkbox" class="theme-controller" value="synthwave" />

            <svg
                class="swap-off size-6fill-current"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
            >
                <path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
            </svg>

            <svg
                class="swap-on size-6 fill-current"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
            >
                <path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
            </svg>
        </label>
    }
}
