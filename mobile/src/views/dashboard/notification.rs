use leptos::prelude::{ElementChild, Get};
use leptos::view;

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;

#[leptos::component]
pub fn NotificationsPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! { <PageHeading text="Notification" /> }>
            <div>the page location is {leptos_router::hooks::use_location().pathname.get()}</div>
        </DashboardLayout>
    }
}
