pub mod authentication;
pub mod dashboard;
pub mod editor;
pub mod walkthrough;
#[leptos::component]
pub fn IndexView() -> impl leptos::IntoView {
    let account_exists = true;
    let access_token_exists = false;
    let is_initialized = true;
    let access_token_expired = true;

    let navigate = leptos_router::hooks::use_navigate();

    if account_exists && access_token_exists {
        navigate("/dashboard", Default::default())
    } else if !(!account_exists || access_token_exists && !access_token_expired) {
        navigate("/auth/login", Default::default())
    } else if !account_exists && !is_initialized {
        navigate("/walkthrough", Default::default())
    } else {
        navigate("/auth/signup", Default::default())
    };
}
