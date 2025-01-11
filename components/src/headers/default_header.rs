use crate::typography::heading::PageHeading;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use thaw::Icon;

#[leptos::component]
pub fn DefaultHeader(page_title: &'static str) -> impl leptos::IntoView {
    view! {
        <header class="flex bg-white fixed top-0 py-4  left-0 mb-5 right-0 w-full border-b gap-2 px-4 items-center  dark:bg-gray-900/70 dark:border-none z-[1000]">
            <a href="/dashboard" class="size-5 font-bold text-black">
                <Icon icon=icondata::OcChevronLeftLg class="size-5" />
            </a>
            <PageHeading text=page_title />
        </header>
    }
}
