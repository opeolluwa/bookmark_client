use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

use bookmark_components::editor::mobile_editor::MarkdownEditor;
use bookmark_components::typography::heading::PageHeading;
use thaw::Icon;

#[leptos::component]
pub fn NewBookmarkPage() -> impl leptos::IntoView {
    view! {
        <header class="flex bg-white fixed top-0 py-4  left-0 mb-5 right-0 w-full border-b gap-2 px-4 items-center  dark:bg-gray-900/70 dark:border-none z-[1000]">
            <a href="/dashboard" class="size-5 font-bold text-black">
                <Icon icon=icondata::OcChevronLeftLg class="size-5" />
            </a>
            <PageHeading text="New Bookmark" />
        </header>

        <MarkdownEditor />

        <button class="fab">
            <Icon icon=icondata::HiArrowDownOnSquareStackOutlineLg class="size-5" />
        </button>
    }
}
