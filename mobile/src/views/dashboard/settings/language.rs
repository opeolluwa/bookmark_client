use bookmark_components::icons::language_icon::LanguageIcon;
use bookmark_components::settings::Language;
use bookmark_components::settings::SettingsRoute;
use leptos::prelude::*;
use leptos::view;
use thaw::{Menu, MenuItem, MenuTrigger, MenuTriggerType};

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
