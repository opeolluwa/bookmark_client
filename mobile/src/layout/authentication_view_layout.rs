use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn AuthenticationLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! { <div class="px-4 pt-6 relative w-full h-screen overflow-y-scroll ">{children}</div> }
}
