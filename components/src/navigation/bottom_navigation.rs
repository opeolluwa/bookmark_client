use leptos::either::Either;
use leptos::prelude::{ClassAttribute, ElementChild, Get};
use leptos::{view, IntoView};
use leptos_heroicons::size_24::outline::{Bell, Cog6Tooth, Home, Sparkles, User};
use leptos_heroicons::size_24::solid::{
    Bell as SolidBell, Cog6Tooth as SolidCog6Tooth, Home as SolidHome, Sparkles as SolidSparkles,
    User as SolidUser,
};
use leptos_router::hooks::use_location;

use crate::icon::HeroIcon;

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
    let current_page_route = use_location().pathname.get();
    let page_route = if href.is_empty() {
        "/dashboard".to_string()
    } else {
        format!("/dashboard/{}", href)
    };

    let is_active_route: bool = page_route == current_page_route;

    let menu_class = if is_active_route {
        "flex flex-col items-center p-0 m-0  rounded-lg text-app btn-animated"
    } else {
        "flex flex-col items-center p-0 m-0  rounded-lg hover:text-app btn-animated"
    };
    let menu_icon = if is_active_route {
        Either::Left(alternate_icon)
    } else {
        Either::Right(icon)
    };

    view! {
        <a href=page_route class=menu_class>
            <HeroIcon icon_data=menu_icon />
            <span class="text-[12px] font-medium  capitalize">{label}</span>

        </a>
    }
}

#[leptos::component]

pub fn BottomNavigation() -> impl leptos::IntoView {
    // Define icons
    let settings_icon = view! { <Cog6Tooth /> };
    let home_icon = view! { <Home /> };
    let star_icon = view! { <Sparkles /> };
    let bell_icon = view! { <Bell /> };
    let profile_icon = view! { <User /> };

    let solid_settings_icon = view! { <SolidCog6Tooth /> };
    let solid_home_icon = view! { <SolidHome /> };
    let solid_star_icon = view! { <SolidSparkles /> };
    let solid_bell_icon = view! { <SolidBell /> };
    let solid_profile_icon = view! { <SolidUser /> };

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
