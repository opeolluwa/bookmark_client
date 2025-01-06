use bookmark_components::loaders::loader_dots::LoaderDots;
use leptos::either::Either;
use leptos::prelude::{OnAttribute, Set};
use leptos::prelude::{RwSignal, StyleAttribute};
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;
use leptos_router::hooks::use_navigate;

use crate::app_state::cached_user::CachedUser;

#[leptos::component]
pub fn LoginPage() -> impl leptos::IntoView {
    let cached_user_data_exists = CachedUser::read_state().user.is_some();
    let _cached_user_data = CachedUser::read_state().user;
    let (account_exists, set_account_exists) = signal(cached_user_data_exists);

    let open_loader = RwSignal::new(false);
    let submit_form = move || {
        open_loader.set(true);
        std::thread::sleep(std::time::Duration::from_secs(5));
        open_loader.set(false);

        let navigate = use_navigate();
        navigate("/dashboard", Default::default());
    };

    view! {
        <div class="relative " style="height:calc(100vh - 100px)">
            {if account_exists.get() {
                Either::Right(
                    view! {
                        <div class="mb-12 flex justify-between items-center">
                            <a href="/auth/sign-up" class="block size-6">

                                <ArrowLongLeftIcon />
                            </a>
                            <button
                                on:click=move |_| {
                                    set_account_exists.set(false);
                                }
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
                            <a href="/auth/sign-up">
                                <ArrowLongLeftIcon />
                            </a>
                        </div>
                    },
                )
            }}
            {if account_exists.get() {
                Either::Right(
                    view! {
                        <div class="mb-6">
                            <Heading class="font-bold">Welcome, Adeoye</Heading>
                            <SmallText class="leading-1 text-gray-400">
                                Enter your password to log in
                            </SmallText>
                        </div>
                    },
                )
            } else {
                Either::Left(
                    view! {
                        <div class="mb-6">
                            <Heading class="font-bold">Log in</Heading>
                            <SmallText class="leading-1 mt-2 text-sm small-text">
                                Enter your email and password to log in
                            </SmallText>
                        </div>
                    },
                )
            }}
            <form class="flex flex-col gap-y-4 mt-6">
                {if !account_exists.get() {
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
                <button
                    on:click=move |event| {
                        event.prevent_default();
                        submit_form();
                    }
                    type="submit"
                    class="btn w-full rounded-lg py-4 bg-app-600 text-white font-medium"
                >
                    Continue
                </button>
            </form>
            <a href="/auth/forgotten-password" class="text-app block  text-sm font-bold mt-3">
                Forgotten password?
            </a>

        </div>

        <LoaderDots open_loader />
    }
}
