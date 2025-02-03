use leptos::view;

use bookmark_components::headers::default_header::DefaultHeader;

#[leptos::component]
pub fn CreateBookmarkCollectionPage() -> impl leptos::IntoView {
    view! { <DefaultHeader page_title="Create Collection" /> }
}
