use leptos::prelude::{Children, ElementChild};
use leptos::IntoView;
use leptos::{prelude::ClassAttribute, view};

#[leptos::component]
pub fn DashboardLayout<F>(children: Children, header_component: F) -> impl leptos::IntoView
where
    F: IntoView,
{
    let children = children();
    view! {
        <div class="relative">
            <header class="flex bg-white fixed top-0 py-4 text-gray-500/90 left-0 mb-5 right-0 w-full border-b px-4 justify-between items-center text-gray-600 dark:bg-gray-900 dark:border-none z-[1000]">
                {header_component}
            </header>

            <div>{children}</div>
        </div>
    }
}
