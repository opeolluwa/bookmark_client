use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{prelude::Children, view, IntoView};
use thaw::Text;

#[leptos::component]
pub fn Heading(#[prop(optional)] class: &'static str, children: Children) -> impl IntoView {
    view! { <h2 class=format!("font-semibold text-2xl {class}")>{children()}</h2> }
}

#[leptos::component]
pub fn PageHeading(#[prop(optional)] text: &'static str) -> impl IntoView {
    view! {
        <Text class="font-medium leading-2 text-xl capitalize text-gray-700 dark:text-gray-400 dark:text-gray-500/90 block">
            {text}
        </Text>
    }
}
