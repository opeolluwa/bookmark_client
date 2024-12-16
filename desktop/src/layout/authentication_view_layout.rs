use leptos::prelude::{Children, ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn AuthenticationLayout(children: Children) -> impl leptos::IntoView {
    let children = children();

    view! {
        <div class="h-screen flex justify-center items-center flex-col bg-gray-50 absolute w-full bg-app-50/20 ">
            <div class="w-[60%] py-12 bg-white rounded-lg shadow px-8 flex flex-col items-center justify-center">
                {children}
            </div>
        </div>
    }
}
