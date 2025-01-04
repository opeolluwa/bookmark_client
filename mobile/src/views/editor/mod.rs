pub mod new_bookmark;

use leptos::view;
use leptos_router::components::Outlet;

use bookmark_components::layouts::mobile::editor::EditorLayout;

#[leptos::component]
pub fn EditorUI() -> impl leptos::IntoView {
    view! {
        <EditorLayout>
            <Outlet />
        </EditorLayout>
    }
}
