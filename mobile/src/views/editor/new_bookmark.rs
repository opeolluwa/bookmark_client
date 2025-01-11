use bookmark_components::headers::default_header::DefaultHeader;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use thaw::Icon;

use bookmark_components::editor::mobile_editor::MarkdownEditor;

#[leptos::component]
pub fn NewBookmarkPage() -> impl leptos::IntoView {
    view! {
        <DefaultHeader page_title="New Bookmark" />

        <MarkdownEditor />

        <button class="fab text-white">
            <Icon icon=icondata::HiArrowDownOnSquareStackOutlineLg class="size-5" />
        </button>
    }
}
