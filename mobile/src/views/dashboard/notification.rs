use crate::layout::dashboard_layout::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;
use leptos::prelude::{ElementChild, Get};
use leptos::view;

#[leptos::component]
pub fn NotificationsPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! { <PageHeading text="Notification" /> }>
            <div>the page location is {leptos_router::hooks::use_location().pathname.get()}</div>
        </DashboardLayout>
    }
}
