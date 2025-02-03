pub mod feature;
pub mod get_started;
pub mod welcome;

use leptos::view;
use leptos_router::components::Outlet;

use bookmark_components::layouts::mobile::walkthrough::WalkthroughLayout;

#[leptos::component]
pub fn WalkthroughUI() -> impl leptos::IntoView {
    view! {
        <WalkthroughLayout>

            <Outlet />

        </WalkthroughLayout>
    }
}
