use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn PlusIcon() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="size-6"
        >
            <path
                fill-rule="evenodd"
                d="M12 3.75a.75.75 0 0 1 .75.75v6.75h6.75a.75.75 0 0 1 0 1.5h-6.75v6.75a.75.75 0 0 1-1.5 0v-6.75H4.5a.75.75 0 0 1 0-1.5h6.75V4.5a.75.75 0 0 1 .75-.75Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[leptos::component]
pub fn PlusIconCircle() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            class="size-5"
        >
            <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm.75-11.25a.75.75 0 0 0-1.5 0v2.5h-2.5a.75.75 0 0 0 0 1.5h2.5v2.5a.75.75 0 0 0 1.5 0v-2.5h2.5a.75.75 0 0 0 0-1.5h-2.5v-2.5Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}
