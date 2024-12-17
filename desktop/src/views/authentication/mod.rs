use leptos::view;
use leptos_router::components::Outlet;

use crate::layout::authentication_view_layout::AuthenticationLayout;

pub mod forgotten_password;
pub mod login;
pub mod set_new_password;
pub mod sign_up;

#[leptos::component]
pub fn AuthenticationUI() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout>
            <Outlet />
        </AuthenticationLayout>
    }
}
