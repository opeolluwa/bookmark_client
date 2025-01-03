use leptos::prelude::{ClassAttribute, ElementChild, StyleAttribute};
use leptos::{children::Children, view};

use crate::navigation::bottom_navigation::BottomNavigation;

#[leptos::component]
pub fn DefaultLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! {
        <div class="relative h-[95vh] overflow-y-hidden w-full bg-white dark:bg-base-100">
            <main
                class="px-4 my-6 pt-12 pb-24 overflow-y-scroll dark:text-gray-400/90 bg-gray-100/80"
                style="overflow-y:scroll; height:100% "
            >
                {children}
            </main>
            <nav class="fixed pt-4 mb-0 fixed bottom-0 z-50  bg-white border-t border-gray-200 text-gray-500 shadow-gray-400 w-full left-0 right-0 py-3 dark:bg-gray-900/50 border-t">
                <BottomNavigation />
            </nav>
        </div>
    }
}
