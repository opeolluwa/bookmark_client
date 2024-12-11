use leptos::{component, view};
use leptos_router::{components::*, path};

use crate::views::authentication::{
    forgotten_password::ForgottenPasswordPage, login::LoginPage, sign_up::SignUpPage,
    AuthenticationUI,
};

#[component]
pub fn MobileApplication() -> impl leptos::IntoView {
    view! {
        <Router>

            <Routes fallback=|| "Page not found">
                <ParentRoute path=path!("/") view=AuthenticationUI>
                    <Route path=path!("") view=LoginPage />
                    <Route path=path!("sign-up") view=SignUpPage />
                    <Route path=path!("forgotten-password") view=ForgottenPasswordPage />
                // <Route path=path!("/set-new-password") view=SetNewPasswordPage />
                </ParentRoute>

            // <ParentRoute path=path!("/dashboard") view=DashboardUI>
            // <Route path=path!("") view=HomePage />
            // <Route path=path!("/calendar") view=CalendarPage />
            // <Route path=path!("/settings") view=SettingsPage />
            // <Route path=path!("students") view=PeoplePage />
            // <Route path=path!("/new-entry") view=NewEntry />
            // </ParentRoute>
            </Routes>
        </Router>
    }
}
