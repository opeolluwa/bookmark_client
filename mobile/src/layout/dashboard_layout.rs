use leptos::{prelude::ClassAttribute, view};

use leptos::prelude::{Children, ElementChild};
use thaw::Text;

#[leptos::component]
pub fn DashboardLayout(
    children: Children,
    #[prop(optional)] title: String,
) -> impl leptos::IntoView {
    let children = children();
    view! {
        <div class="relative">
            <header class="flex bg-white fixed top-0 py-4 text-gray-500/90 left-0 mb-5 right-0 w-full border-b px-4 justify-between items-center text-gray-600 dark:bg-gray-900 dark:border-none z-[1000]">
                <Text class="font-medium leading-2 text-xl text-gray-700 dark:text-gray-400 dark:text-gray-500/90">
                    {title}
                </Text>
            </header>

            <div class="pb-12">{children}</div>
        </div>
    }
}
