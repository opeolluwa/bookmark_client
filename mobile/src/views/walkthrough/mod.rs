use bookmark_components::layouts::mobile::walkthrough::WalkthroughLayout;
use leptos::view;
use leptos_router::components::Outlet;

pub mod feature;
pub mod get_started;
pub mod welcome;

#[leptos::component]
pub fn WalkthroughUI() -> impl leptos::IntoView {
    view! {
        <WalkthroughLayout>

            <Outlet />

        </WalkthroughLayout>
    }
}
