use bookmark_state::{cached_user::CachedUser, installation_status::InstallationStatus};

#[leptos::component]
pub fn IndexView() -> impl leptos::IntoView {
    let account_exists = CachedUser::read_state().user.is_some();
    let app_is_initialized = InstallationStatus::read_state().is_installed;

    let navigate = leptos_router::hooks::use_navigate();

    if !app_is_initialized {
        navigate("/walkthrough", Default::default())
    } else if account_exists {
        navigate("/dashboard", Default::default())
    } else {
        navigate("/auth/login", Default::default())
    }
}
