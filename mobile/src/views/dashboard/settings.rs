use leptos::{
    prelude::{ClassAttribute, CollectView, ElementChild},
    view,
};
use thaw::{Icon, Menu, MenuItem, MenuTrigger, MenuTriggerType};

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::{
    icons::language_icon::LanguageIcon,
    settings::{Language, SettingsRoute, SettingsTab},
    typography::heading::PageHeading,
};

#[leptos::component]
pub fn SettingsPage() -> impl leptos::IntoView {
    let language_select_component = LanguageComponent();
    let color_theme_swap_component = ColorThemeSwapComponent();

    view! {
        <DashboardLayout header_component=view! { <PageHeading text="settings" /> }>

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
        <Menu
            on_select
            trigger_type=MenuTriggerType::Hover
            position=thaw::MenuPosition::BottomStart
            class="w-full"
        >
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
pub fn ColorThemeSwapComponent() -> impl leptos::IntoView {
    view! {
        <label class="swap swap-rotate">
            <input type="checkbox" class="theme-controller" value="synthwave" />
            <Icon icon=icondata::BsSun class="swap-off size-5 fill-current" />
            <Icon icon=icondata::BsMoon class="swap-on size-5 fill-current" />
        </label>
    }
}
