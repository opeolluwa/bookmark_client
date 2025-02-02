use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn WalkthroughLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! { <div class="px-4 py-6 relative w-full overflow-y-hidden">{children}</div> }
}
