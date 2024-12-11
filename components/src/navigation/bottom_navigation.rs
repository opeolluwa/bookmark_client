use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

use leptos_heroicons::size_24::outline::{Calendar, Cog6Tooth, Home, Plus, User};

use crate::icon::HeroIcon;

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

            <HeroIcon icon_data=icon />
            <span class="text-sm capitalize sr-only">{label}</span>
        </a>
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    let settings_icon = Cog6Tooth();
    let home_icon = Home();
    let calendar_icon = Calendar();
    let plus_icon = Plus();
    let people_icon = User();

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
