use bookmark_components::typography::heading::PageHeading;
use leptos::view;

use crate::layout::dashboard_layout::DashboardLayout;

#[leptos::component]
pub fn FavoritesPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! {
            <PageHeading text="Favorites" />
        }>fav</DashboardLayout>
    }
}
