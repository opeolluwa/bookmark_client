pub mod favorites;
pub mod home;
pub mod notification;
pub mod profile;
pub mod search;
pub mod settings;

use leptos::view;
use leptos_router::components::Outlet;

use bookmark_components::layouts::mobile::default::DefaultLayout;

#[leptos::component]
pub fn DashboardUI() -> impl leptos::IntoView {
    view! {
        <DefaultLayout>
            <Outlet />
        </DefaultLayout>
    }
}
