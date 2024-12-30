use bookmark_components::layouts::mobile::editor::EditorLayout;
use leptos::view;
use leptos_router::components::Outlet;

#[leptos::component]
pub fn EditorUI() -> impl leptos::IntoView {
    view! {
        <EditorLayout>
            <Outlet />
        </EditorLayout>
    }
}
