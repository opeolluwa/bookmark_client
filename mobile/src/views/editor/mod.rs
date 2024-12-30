use bookmark_components::layouts::mobile::editor::EditorLayout;
use leptos::view;
use leptos_router::components::Outlet;

pub mod new_bookmark;
#[leptos::component]
pub fn EditorUI() -> impl leptos::IntoView {
    view! {
        <EditorLayout>
            <Outlet />
        </EditorLayout>
    }
}
