use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{view, IntoView};

#[leptos::component]
pub fn HeroIcon<F>(#[prop(optional)] class: &'static str, icon_data: F) -> impl leptos::IntoView
where
    F: IntoView,
{
    view! { <span class=format!("size-6 {class}")>{icon_data}</span> }
}
