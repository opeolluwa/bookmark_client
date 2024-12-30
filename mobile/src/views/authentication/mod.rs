use bookmark_components::layouts::mobile::authentication::AuthenticationLayout;
use leptos::view;
use leptos_router::components::Outlet;

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
