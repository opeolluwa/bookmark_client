use bookmark_components::icon::HeroIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;
use leptos::either::Either;
use leptos::prelude::{OnAttribute, Set};
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};
use leptos_heroicons::size_24::outline::ArrowLongLeft;

#[leptos::component]
pub fn LoginPage() -> impl leptos::IntoView {
    let (account_exists, set_account_exists) = signal(true);

    let (is_loading, _set_is_loading) = signal(false);

    view! {
        <div class="">
            {if account_exists.get() == true {
                Either::Right(
                    view! {
                        <div class="mb-12 flex justify-between items-center">
                            // <a href="sign-up">
                                <HeroIcon icon_data=ArrowLongLeft />
                            // </a>
                            <button
                                on:click=move |_| set_account_exists.set(false)
                                class="font-medium text-sm  text-gray-600"
                            >
                                Not Adeoye?
                            </button>
                        </div>
                    },
                )
            } else {
                Either::Left(
                    view! {
                        <div class="mb-12 flex justify-between items-center">
                            <button on:click=move |_| {
                                set_account_exists.set(false)
                            }>// <ArrowLongLeftIcon />
                            </button>
                        </div>
                    },
                )
            }}
            {if account_exists.get() == true {
                Either::Right(
                    view! {
                        <div class="mb-6">
                            <Heading class="font-bold".into()>Welcome, Adeoye</Heading>
                            <SmallText class="leading-1 text-gray-400"
                                .into()>Enter your email and password to log in</SmallText>
                        </div>
                    },
                )
            } else {
                Either::Left(
                    view! {
                        <div class="mb-6">
                            <Heading class="font-bold".into()>Log in</Heading>
                            <SmallText class="leading-1 mt-2 text-sm small-text">
                                Enter your email and password to log in
                            </SmallText>
                        </div>
                    },
                )
            }}
            <form class="flex flex-col gap-y-4 mt-6">
                {if account_exists.get() == false {
                    Either::Right(
                        view! {
                            <div class="form-input">
                                <label for="email">Email</label>
                                <input type="email" placeholder="type your email" />
                            </div>
                            <div class="form-input">
                                <label for="password">Password</label>
                                <input type="password" placeholder="type your password" />
                            </div>
                        },
                    )
                } else {
                    Either::Left(
                        view! {
                            <div class="form-input">
                                <label for="password">Password</label>
                                <input type="password" placeholder="type your password" />
                            </div>
                        },
                    )
                }}
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
            </form> <a href="/forgotten-password" class="text-app block  text-sm font-bold mt-3">
                Forgotten password?
            </a>
            <div class="flex items-center justify-center  fixed z-50  bottom-24 left-0 right-0">
                <button class="btn  shadow-none bg-app-50 border-none ">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="size-6 text-app bg-app-50/50"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M7.864 4.243A7.5 7.5 0 0 1 19.5 10.5c0 2.92-.556 5.709-1.568 8.268M5.742 6.364A7.465 7.465 0 0 0 4.5 10.5a7.464 7.464 0 0 1-1.15 3.993m1.989 3.559A11.209 11.209 0 0 0 8.25 10.5a3.75 3.75 0 1 1 7.5 0c0 .527-.021 1.049-.064 1.565M12 10.5a14.94 14.94 0 0 1-3.6 9.75m6.633-4.596a18.666 18.666 0 0 1-2.485 5.33"
                        />
                    </svg>

                </button>
            </div>
        </div>
    }
}
