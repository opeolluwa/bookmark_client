use filesync_icons::calendar_icon::CalendarIconOutline;
use filesync_icons::home_icon::HomeIcon;
use filesync_icons::plus_icon::PlusIcon;
use filesync_icons::settings_icon::SettingsIconOutline;
use filesync_icons::user_icon::UserIconMultipleOutline;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

#[leptos::component]
pub fn BottomNavigationRoute<F>(
    label: &'static str,
    href: &'static str,
    icon: F,
) -> impl leptos::IntoView
where
    F: IntoView,
{
    view! {
        <a
            href=format!("/dashboard/{href}")
            class="flex flex-col items-center p-0 m-0  w-full rounded-lg size-6"
        >
            {icon}
            <span class="text-sm capitalize sr-only">{label}</span>
        </a>
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    let settings_icon = SettingsIconOutline();
    let home_icon = HomeIcon();
    let calendar_icon = CalendarIconOutline();
    let plus_icon = PlusIcon();
    let people_icon = UserIconMultipleOutline();

    view! {
        <nav class="btm-nav">
            <BottomNavigationRoute label="home" href="" icon=home_icon />
            <BottomNavigationRoute label="calendar" href="calendar" icon=calendar_icon />
            <BottomNavigationRoute label="add new" href="new-entry" icon=plus_icon />
            <BottomNavigationRoute label="people" href="students" icon=people_icon />
            <BottomNavigationRoute label="settings" href="settings" icon=settings_icon />

        </nav>
    }
}
