use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{prelude::Children, view};

#[leptos::component]
pub fn View(children: Children, #[prop(optional)] class: &'static str) -> impl leptos::IntoView {
    view! { <div class="class">{children()}</div> }
}
