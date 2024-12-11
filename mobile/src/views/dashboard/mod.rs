use leptos::view;
use leptos_router::components::Outlet;

use crate::layout::default_layout::DefaultLayout;

pub mod favorites;
pub mod home;
pub mod settings;

#[leptos::component]
pub fn DashboardUI() -> impl leptos::IntoView {
    view! {
        <DefaultLayout>
            <Outlet />
        </DefaultLayout>
    }
}
