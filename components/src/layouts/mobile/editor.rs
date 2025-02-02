use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn EditorLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! {
        <div class="px-4 pt-20 relative w-full h-[95vh] overflow-y-scroll  bg-gray-100/80 ">
            {children}
        </div>
    }
}
