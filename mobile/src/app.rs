use leptos::{component, view};
use leptos_router::{components::*, path};

use crate::views::{
    authentication::{
        forgotten_password::ForgottenPasswordPage, login::LoginPage,
        set_new_password::SetNewPasswordPage, sign_up::SignUpPage, AuthenticationUI,
    },
    dashboard::{
        favorites::FavoritesPage, home::HomePage, notification::NotificationsPage,
        profile::UserAccountPage, search::SearchPage, settings::SettingsPage, DashboardUI,
    },
    walkthrough::{
        feature::FeaturePage, get_started::GetStartedPage, welcome::WelcomePage, WalkthroughUI,
    },
};

#[component]
pub fn MobileApplication() -> impl leptos::IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <ParentRoute path=path!("/dd") view=WalkthroughUI>
                    <Route path=path!("/") view=WelcomePage />
                    <Route path=path!("/feature") view=FeaturePage />
                    <Route path=path!("/get-started") view=GetStartedPage />
                </ParentRoute>

                <ParentRoute path=path!("/a") view=AuthenticationUI>
                    <Route path=path!("login") view=LoginPage />
                    <Route path=path!("sign-up") view=SignUpPage />
                    <Route path=path!("forgotten-password") view=ForgottenPasswordPage />
                    <Route path=path!("/set-new-password") view=SetNewPasswordPage />
                </ParentRoute>

                <ParentRoute path=path!("") view=DashboardUI>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("/favorites") view=FavoritesPage />
                    <Route path=path!("settings") view=SettingsPage />
                    <Route path=path!("search") view=SearchPage />
                    <Route path=path!("notifications") view=NotificationsPage />
                    <Route path=path!("profile") view=UserAccountPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
