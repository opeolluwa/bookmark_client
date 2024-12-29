use leptos::view;

pub mod authentication;
pub mod dashboard;
pub mod walkthrough;

// decide which UI to show based on the available data
#[leptos::component]
pub fn AuthenticationIndexView() -> impl leptos::IntoView {
    view! {}
}

#[leptos::component]
pub fn DashboardIndexView() -> impl leptos::IntoView {
    view! {}
}

#[leptos::component]
pub fn WalkthroughIndexView() -> impl leptos::IntoView {
    view! {}
}

#[leptos::component]
pub fn IndexView() -> impl leptos::IntoView {
    let account_exists = true;
    let access_token_exists = true;
    let is_authenticated = account_exists && access_token_exists;

    view! {
        
    }
}
