use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

use leptos_heroicons::size_24::outline::{Cog6Tooth, Home, Sparkles};

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
    let star_icon = Sparkles();

    view! {
        <nav class="btm-nav">
            <BottomNavigationRoute label="home" href="" icon=home_icon />

            <BottomNavigationRoute label="favorites" href="favorites" icon=star_icon />

            <BottomNavigationRoute label="settings" href="settings" icon=settings_icon />

        </nav>
    }
}
