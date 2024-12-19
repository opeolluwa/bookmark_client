use bookmark_components::layout::view::View;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn GetStartedPage() -> impl leptos::IntoView {
    view! {
        <View class="flex flex-col relative h-[90vh] overflow-hidden">
            <div>
                <img src="/assets/illustrations/get-started.png" alt="welcome" />
            </div>
            <div class="text-center ">
                <h1 class="text-3xl font-black">Get started!</h1>
                <p class="leading-2 text-[18px] text-gray-400 mt-2">
                 Create a free account or proceed without sign up
                </p>
            </div>

            <div class="flex flex-col mt-6">
                <a href="/dashboard" class="btn text-white bg-app  border-none">Get Started</a>

                <a href ="/auth/login" class="btn  bg-gray-50/50 mt-2">Create Account</a>
            </div>

            <div class="flex justify-between absolute w-full bottom-2 left-0  right-0">
                <a href="/feature" class="btn btn-sm">
                    Prev
                </a>

            </div>
        </View>
    }
}
