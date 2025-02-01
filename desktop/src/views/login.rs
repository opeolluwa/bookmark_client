use leptos::either::Either;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;

#[leptos::component]
pub fn LoginPage() -> impl leptos::IntoView {
    let (is_loading, _set_is_loading) = signal(false);

    view! {
        <div class="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full bg-[url(/assets/background/topography.svg)] bg-no-repeat bg-gray-040 bg-cover bg-blend-multiply">
            <div class="pb-6 py-2 px-6 w-[40%] bg-white rounded-lg shadow-lg">
                <div class="relative">
                    <div class="m-4 text-center">
                        <Heading class="font-bold ">Welcome back!</Heading>
                        <SmallText class="leading-1 mt-2 text-sm small-text">
                            Sign in to continue
                        </SmallText>
                    </div>

                    <form class="flex flex-col gap-y-4 mt-6">

                         <div class="form-input">
                                <label for="email">Email</label>
                                <input
                                    type="email"
                                    placeholder="type your email"
                                />
                            </div>
                            <div class="form-input">
                                <label for="password">Password</label>
                                <input
                                    type="password"
                                    placeholder="type your password"
                                />
                            </div>

                        <button
                            type="submit"
                            class="btn border-none hover:bg-app-600/80 w-full rounded-lg py-4 bg-app-600 text-white font-medium"
                        >
                            {if is_loading.get() {
                                Either::Right(
                                    view! { <span class="loading loading-ring loading-sm"></span> },
                                )
                            } else {
                                Either::Left("Continue")
                            }}

                        </button>
                    // href="/mobile/authentication/forgotten-password"
                    </form>
                    <a
                        href="/forgotten-password"
                        class="text-gray-400 block text-center text-sm  mt-6 dark:text-gray-500 flex justify-end"
                    >
                        Forgotten password?
                    </a>

                </div>
            </div>
        </div>
    }
}
