use leptos::{component, view};
use leptos_router::{components::*, path};

use crate::views::login::LoginPage;

#[component]
pub fn DesktopApplication() -> impl leptos::IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <Route path=path!("") view=LoginPage />

            </Routes>
        </Router>
    }
}
//
//
