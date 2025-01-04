use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use thaw::Image;

use bookmark_components::view::View;

#[leptos::component]
pub fn FeaturePage() -> impl leptos::IntoView {
    view! {
        <View class="flex flex-col relative h-[90vh] overflow-hidden">
            <div>
                <Image
                    src="/assets/illustrations/feature.png"
                    alt="welcome"
                    width="100%"
                    height="auto"
                />
            </div>
            <div class="text-center">
                <h1 class="text-3xl font-black">Offline first</h1>
                <p class="leading-2 text-[18px] text-gray-400 mt-2">
                    Access all your important bookmarks without internet
                </p>
            </div>

            <div class="flex justify-between absolute w-full bottom-0 mt-6 left-0  right-0">
                <a href="/walkthrough/feature" class="btn btn-sm">
                    Prev
                </a>
                <a href="/walkthrough/get-started" class="btn btn-sm bg-app text-white">
                    Next
                </a>
            </div>
        </View>
    }
}
