use leptos::prelude::{ClassAttribute, CustomAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn ArrowLeftRightIconSolid() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class=format!("size-6")
        >
            <path d="M16 16V12L21 17L16 22V18H4V16H16ZM8 2V5.999L20 6V8H8V12L3 7L8 2Z"></path>
        </svg>
    }
}

#[leptos::component]
pub fn ArrowLongLeftIcon() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6.75 15.75 3 12m0 0 3.75-3.75M3 12h18"
            />
        </svg>
    }
}

#[leptos::component]
pub fn ArrowLongRightIcon() -> impl leptos::IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="size-6"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"
            />
        </svg>
    }
}
