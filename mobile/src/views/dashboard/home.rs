use crate::layout::dashboard_layout::DashboardLayout;
use bookmark_components::icon::HeroIcon;
use bookmark_components::typography::heading::PageHeading;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_heroicons::size_24::outline::Bars3;

#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    view! { <DashboardLayout header_component=HomePageHeader>homeeeeeee</DashboardLayout> }
}

#[leptos::component]
pub fn HomePageHeader() -> impl leptos::IntoView {
    view! {
        <div class="flex gap-x-2 items-center">
            <HeroIcon icon_data=Bars3 class="size-6" />
            <PageHeading text="Home" />
        </div>
    }
}
