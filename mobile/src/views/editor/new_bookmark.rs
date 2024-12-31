use bookmark_components::icons::chevron::ChevronLeftIcon;
use bookmark_components::typography::heading::PageHeading;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[leptos::component]
pub fn NewBookmarkPage() -> impl leptos::IntoView {
    view! {
        <header class="flex bg-white fixed top-0 py-4  left-0 mb-5 right-0 w-full border-b gap-2 px-4 items-center  dark:bg-gray-900/70 dark:border-none z-[1000]">
            <a href="/dashboard">
                <ChevronLeftIcon />
            </a>
            <PageHeading text="New Bookmark" />
        </header>
    }
}
