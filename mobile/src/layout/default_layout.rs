use bookmark_components::navigation::bottom_navigation::BottomNavigation;
use leptos::prelude::{ClassAttribute, ElementChild, StyleAttribute};
use leptos::{children::Children, view};

#[leptos::component]
pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! {
        <div class="relative h-[98vh] overflow-y-hidden w-full bg-white dark:bg-base-100">
            <main
                class="px-4 my-6 pt-6 overflow-y-scroll my-4 dark:text-gray-400/90 bg-gray-100"
                style="height:100%; overflow-y:scroll "
            >
                {children}
            </main>
            <nav class="fixed pt-4 mb-0 fixed bottom-0 z-50  bg-white border-t border-gray-200 text-gray-500/90 shadow-gray-400 w-full left-0 right-0 py-3 dark:bg-gray-900/50 border-t">
                <BottomNavigation />
            </nav>
        </div>
    }
}
