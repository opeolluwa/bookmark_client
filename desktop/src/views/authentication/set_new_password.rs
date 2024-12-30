use leptos::either::Either;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;

#[leptos::component]
pub fn SetNewPasswordPage() -> impl leptos::IntoView {
    let (is_loading, _set_is_loading) = signal(false);

    view! {
        <div class="">
            <a href="/" class="mb-12 flex justify-between items-center">
                <ArrowLongLeftIcon />
            </a>

            <div class="mb-6">
                <Heading class="font-bold">"Set a new password"</Heading>
                <SmallText class="leading-1 text-gray-400">"Create a new password"</SmallText>
            </div>

            <form class="flex flex-col gap-y-4 mt-6">
                <div class="form-input">
                    <label for="password">New password</label>
                    <input type="password" placeholder="new password" />
                </div>
                <div class="form-input">
                    <label for="password">Confirm new password</label>
                    <input type="password" placeholder="confirm password" />
                </div>
                <a
                    href="/dashboard"
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

                </a>
            </form>
            <SmallText class="text-app-400 block text-sm font-bold mt-3">
                Remember your password? Login!
            </SmallText>
        </div>
    }
}
