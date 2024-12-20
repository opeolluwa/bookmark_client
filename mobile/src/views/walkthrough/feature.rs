use bookmark_components::layout::view::View;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn FeaturePage() -> impl leptos::IntoView {
    view! {
        <View class="flex flex-col relative h-[90vh] overflow-hidden">
            <div>
                <img src="/assets/illustrations/feature.png" alt="welcome" />
            </div>
            <div class="text-center">
                <h1 class="text-3xl font-black">Offline first</h1>
                <p class="leading-2 text-[18px] text-gray-400 mt-2">
                    Access all your important bookmarks without internet
                </p>
            </div>

            <div class="flex justify-between absolute w-full bottom-0 mt-6 left-0  right-0">
                <a href="/" class="btn btn-sm">
                    Prev
                </a>
                <a href="/get-started" class="btn btn-sm bg-app text-white">
                    Next
                </a>
            </div>
        </View>
    }
}
