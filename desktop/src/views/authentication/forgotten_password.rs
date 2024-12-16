use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;

use leptos::either::Either;

use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

#[leptos::component]
pub fn ForgottenPasswordPage() -> impl leptos::IntoView {
    let (is_loading, _set_is_loading) = signal(false);

    view! {
        <div class="">

            <a href="/" class="mb-12 flex justify-between items-center">
                <ArrowLongLeftIcon />
            </a>

            <div class="mb-6">
                <Heading class="font-bold".into()>"Let's get you back in"</Heading>
                <SmallText class="leading-1 text-gray-400">
                    "Provide the email associated with your account and we'd send more instruction"
                </SmallText>
            </div>

            <form class="flex flex-col gap-y-4 mt-6">
                <div class="form-input">
                    <label for="email">Email</label>
                    <input type="email" placeholder="type your email" />
                </div>

                <a
                    href="/set-new-password"
                    type="submit"
                    class="btn w-full rounded-lg py-4 bg-app-600 text-white font-medium"
                >
                    {if is_loading.get() == true {
                        Either::Right(
                            view! { <span class="loading loading-ring loading-sm"></span> },
                        )
                    } else {
                        Either::Left("Continue")
                    }}

                </a>
            </form>
            <a href="/" class="text-app block text-sm font-bold mt-3">
                "Remember your password? Login!"
            </a>
        </div>
    }
}
