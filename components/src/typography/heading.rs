use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{prelude::Children, view, IntoView};

#[leptos::component]
pub fn Heading(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <h2 class=format!("font-semibold text-2xl {class}")>{children()}</h2> }
}
