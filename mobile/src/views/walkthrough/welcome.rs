use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use thaw::Image;

use bookmark_components::view::View;

#[leptos::component]
pub fn WelcomePage() -> impl leptos::IntoView {
    view! {
        <View class="flex flex-col relative h-[90vh] overflow-hidden">
            <div>
                <Image
                    width="100%"
                    height="auto"
                    src="/assets/illustrations/welcome.png"
                    alt="welcome"
                />
            </div>
            <div class="text-center">
                <h1 class="text-3xl font-black">Awesome bookmarks</h1>
                <p class="leading-2 text-[18px] text-gray-400 mt-2">
                    Craft, curate and share all important data on a single platform
                </p>
            </div>

            <div class="flex justify-end absolute w-full bottom-2 left-0  right-0">
                <a href="" disabled class="btn hidden  px-4 bg-gray-400 text-black btn-sm">
                    Prev
                </a>
                <a href="/walkthrough/feature" class="btn px-4 btn-sm bg-app text-white">
                    Next
                </a>

            </div>
            " "
        </View>
    }
}
