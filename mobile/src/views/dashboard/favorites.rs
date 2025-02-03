use leptos::view;

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;

#[leptos::component]
pub fn FavoritesPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! { <PageHeading text="Favorites" /> }>
            <div></div>
        </DashboardLayout>
    }
}
