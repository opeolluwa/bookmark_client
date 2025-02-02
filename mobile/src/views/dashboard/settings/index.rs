use leptos::view;

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::{settings::SettingsTab, typography::heading::PageHeading};

use super::{
    auto_login::AutoLoginComponent, language::LanguageComponent, theme::ColorThemeSwapComponent,
};

#[leptos::component]
pub fn SettingsPage() -> impl leptos::IntoView {
    let language_select_component = LanguageComponent();
    let color_theme_swap_component = ColorThemeSwapComponent();
    let auto_login_component = AutoLoginComponent();
    view! {
        <DashboardLayout header_component=view! { <PageHeading text="settings" /> }>

            <SettingsTab
                title="Preferred language"
                component=language_select_component
                class="mt-0"
            />
            <SettingsTab title="Color theme" component=color_theme_swap_component />

            <SettingsTab title="Auto login" component=auto_login_component />
        </DashboardLayout>
    }
}
