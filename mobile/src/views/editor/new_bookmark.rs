use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use leptos_heroicons::size_24::solid::FolderArrowDown as SafeIcon;

use bookmark_components::editor::mobile_editor::MarkdownEditor;
use bookmark_components::icon::HeroIcon;
use bookmark_components::icons::chevron::ChevronLeftIcon;
use bookmark_components::typography::heading::PageHeading;

#[leptos::component]
pub fn NewBookmarkPage() -> impl leptos::IntoView {
    let save_icon = view! { <SafeIcon /> };

    view! {
        <header class="flex bg-white fixed top-0 py-4  left-0 mb-5 right-0 w-full border-b gap-2 px-4 items-center  dark:bg-gray-900/70 dark:border-none z-[1000]">
            <a href="/dashboard">
                <ChevronLeftIcon />
            </a>
            <PageHeading text="New Bookmark" />
        </header>

        <MarkdownEditor />

        <button class="fab">
            <HeroIcon icon_data=save_icon />
        </button>
    }
}
