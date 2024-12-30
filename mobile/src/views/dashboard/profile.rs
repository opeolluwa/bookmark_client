use bookmark_components::typography::heading::PageHeading;
use leptos::view;

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;

#[leptos::component]
pub fn UserAccountPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! {
            <PageHeading text="Profile" />
        }>""</DashboardLayout>
    }
}
