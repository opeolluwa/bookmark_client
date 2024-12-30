pub mod forgotten_password;
pub mod login;
pub mod set_new_password;
pub mod sign_up;

use leptos::view;
use leptos_router::components::Outlet;

use bookmark_components::layouts::mobile::authentication::AuthenticationLayout;

#[leptos::component]
pub fn AuthenticationUI() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout>
            <Outlet />
        </AuthenticationLayout>
    }
}
