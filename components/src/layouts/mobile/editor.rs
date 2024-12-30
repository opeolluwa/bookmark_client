use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn EditorLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! { <div class="px-4 pt-6 relative w-full h-[90vh] overflow-y-scroll ">{children}</div> }
}
