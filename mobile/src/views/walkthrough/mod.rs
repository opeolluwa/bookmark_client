use leptos::view;
use leptos_router::components::Outlet;

use crate::layout::walkthrough::WalkthroughLayout;

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
