use leptos::view;
use leptos_router::components::Outlet;



pub mod login;
pub mod forgot_password;
pub mod set_new_password;

#[leptos::component]
pub fn AuthenticationUI() -> impl leptos::IntoView {
    view! {
        <AuthenticationLayout>
            <Outlet />
        </AuthenticationLayout>
    }
}
