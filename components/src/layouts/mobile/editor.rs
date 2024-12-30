use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn EditorLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! { <div class="px-4 relative w-full h-[90vh] overflow-y-scroll dark:text-gray-400/90 bg-gray-100/80 ">{children}</div> }
}
