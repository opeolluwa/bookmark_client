use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;
use leptos::either::Either;
use leptos::prelude::StyleAttribute;
use leptos::prelude::{OnAttribute, Set};
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

#[leptos::component]
pub fn LoginPage() -> impl leptos::IntoView {
    let (account_exists, set_account_exists) = signal(true);

    let (is_loading, _set_is_loading) = signal(false);

    view! {
        <div class="relative">
            <div class="m-6 text-center">
                <Heading class="font-bold dark:text-gray-500">Welcome back!</Heading>
                <SmallText class="leading-1 mt-2 text-sm small-text">Sign in to continue</SmallText>
            </div>

            <form class="flex flex-col gap-y-4 mt-6">

                <div class="form-input desktop">
                    <label for="email">Email</label>
                    <input type="email" placeholder="type your email" />
                </div>
                <div class="form-input desktop">
                    <label for="password">Password</label>
                    <input type="password" placeholder="type your password" />
                </div>

                <a
                    href="/dashboard"
                    // disabled=is_loading
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
            // href="/mobile/authentication/forgotten-password"
            </form>
            <a
                href="/forgotten-password"
                class="text-gray-400 block text-center text-sm  mt-6 dark:text-gray-500 flex justify-end"
            >
                Forgotten password?
            </a>

        </div>
    }
}
