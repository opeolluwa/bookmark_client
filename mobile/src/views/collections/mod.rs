pub mod new_collection;

use leptos::view;
use leptos_router::components::Outlet;

use bookmark_components::layouts::mobile::default::DefaultLayout;

#[leptos::component]
pub fn BookmarkCollectionUI() -> impl leptos::IntoView {
    view! {
        <DefaultLayout>
            <Outlet />
        </DefaultLayout>
    }
}
