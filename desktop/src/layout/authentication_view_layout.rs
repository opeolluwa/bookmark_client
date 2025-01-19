use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn AuthenticationLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! {
        <div class="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full dark:bg-gray-800">
            <div class="pb-6 py-2 px-6 w-[40%] bg-white dark:bg-gray-900/50 rounded-lg shadow-lg">
                {children}
            </div>
        </div>
    }
}
