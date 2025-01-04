use leptos::either::Either;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;

#[leptos::component]
pub fn SignUpPage() -> impl leptos::IntoView {
    let (is_loading, _set_is_loading) = signal(false);

    view! {
        <div class="">

            <div class="mb-12 flex justify-between items-center">
                <a href="/" class="block size-6">
                    <ArrowLongLeftIcon />
                </a>
            </div>
            <div class="mb-6">
                <Heading class="font-bold">Create account</Heading>
                <SmallText class="leading-1 mt-2 text-sm small-text">
                    Create and account to begin
                </SmallText>
            </div>
            <form class="flex flex-col gap-y-4 mt-6">
                <div class="form-input">
                    <label for="first_name">First name</label>
                    <input type="text" placeholder="type your first name" />
                </div>

                <div class="form-input">
                    <label for="last_name">last name</label>
                    <input type="text" placeholder="type your last name" />
                </div>
                <div class="form-input">
                    <label for="email">Email</label>
                    <input type="email" placeholder="type your email" />
                </div>
                <div class="form-input">
                    <label for="password">Password</label>
                    <input type="password" placeholder="type your password" />
                </div>

                <button
                    disabled=is_loading
                    type="submit"
                    class="btn w-full rounded-lg py-4 bg-app-600 text-white font-medium"
                >
                    {if is_loading.get() {
                        Either::Right(
                            view! { <span class="loading loading-ring loading-sm"></span> },
                        )
                    } else {
                        Either::Left("Continue")
                    }}

                </button>
            </form>
            <a href="/forgotten-password" class="text-app block  text-sm font-bold mt-3">
                "Already have and account? Login"
            </a>

        </div>
    }
}
