use bookmark_components::layout::view::View;
use bookmark_components::typography::heading::PageHeading;
use bookmark_components::typography::small_text::SmallText;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn GetStartedPage() -> impl leptos::IntoView {
    view! {
        <View class="flex flex-col relative h-[80vh]">
            <div>
                <img src="/assets/illustrations/welcome.png" alt="welcome" />
            </div>
            <div class="text-center ">
                <PageHeading text="Lorem Ipsum" />
                <SmallText>
                    Over 500+ professionally designed, fully responsive, expertly crafted component examples you can drop into your Tailwind projects and customize to your hears content.
                </SmallText>
            </div>

            <div class="flex flex-col">
                <button class="btn text-app border-none">Get Started</button>

                <button>Create Account</button>
            </div>

            <div class="flex justify-between absolute w-full bottom-2 left-0  right-0">
                <a href=""  class="btn btn-sm">
                    Prev
                </a>
                <a href="/feature" class="btn btn-sm bg-app text-white">
                    Next
                </a>
            </div>
        </View>
    }
}
