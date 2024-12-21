use crate::icon::HeroIcon;
use leptos::either::Either;
use leptos::prelude::{ClassAttribute, ElementChild, Get};
use leptos::{view, IntoView};
use leptos_heroicons::size_24::outline::{Bell, Cog6Tooth, Home, Sparkles, User};
use leptos_heroicons::size_24::solid::{
    Bell as SolidBell, Cog6Tooth as SolidCog6Tooth, Home as SolidHome, Sparkles as SolidSparkles,
    User as SolidUser,
};
use leptos_router::hooks::use_location;

#[leptos::component]
pub fn BottomNavigationRoute<U, K>(
    label: &'static str,
    href: &'static str,
    icon: U,
    alternate_icon: K,
) -> impl leptos::IntoView
where
    U: IntoView,
    K: IntoView,
{
    let current_page_location = use_location().pathname.get();
    let page_route = format!("/dashboard/{href}");

    view! {
        {if page_route ==current_page_location   {
            Either::Right(
                view! {
                    <a
                        href=page_route
                        class="flex flex-col items-center p-0 m-0  rounded-lg text-app btn-animated"
                    >
                        <HeroIcon icon_data=alternate_icon />
                        <span class="text-sm capitalize sr-only">{label}</span>
                    </a>
                },
            )
        } else {
            Either::Left(
                view! {
                    <a
                        href=page_route
                        class="flex flex-col items-center p-0 m-0  rounded-lg hover:text-app btn-animated"
                    >
                        <HeroIcon icon_data=icon />
                        <span class="text-sm sr-only">{label}</span>
                    </a>
                },
            )
        }}
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    let settings_icon = Cog6Tooth();
    let home_icon = Home();
    let star_icon = Sparkles();
    let bell_icon = Bell();
    let profile_icon = User();

    let solid_settings_icon = SolidCog6Tooth();
    let solid_home_icon = SolidHome();
    let solid_star_icon = SolidSparkles();
    let solid_bell_icon = SolidBell();
    let solid_profile_icon = SolidUser();

    view! {
        <nav class="btm-nav">
            <BottomNavigationRoute
                label="home"
                href=""
                icon=home_icon
                alternate_icon=solid_home_icon
            />

            <BottomNavigationRoute
                label="notification"
                href="notifications"
                icon=bell_icon
                alternate_icon=solid_bell_icon
            />

            <BottomNavigationRoute
                label="favorites"
                href="favorites"
                icon=star_icon
                alternate_icon=solid_star_icon
            />

            <BottomNavigationRoute
                label="profile"
                href="profile"
                icon=profile_icon
                alternate_icon=solid_profile_icon
            />

            <BottomNavigationRoute
                label="settings"
                href="settings"
                icon=settings_icon
                alternate_icon=solid_settings_icon
            />
        </nav>
    }
}
