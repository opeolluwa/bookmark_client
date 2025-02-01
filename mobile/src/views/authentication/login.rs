use bookmark_components::forms::login::{LoginFormData, LoginResponse};
use bookmark_components::forms::user_profile::UserProfile;
use bookmark_components::forms::FormResponse;
use bookmark_components::loaders::loader_dots::LoaderDots;
use bookmark_state::cached_user::CachedUser;
use leptos::either::Either;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::{NodeRef, NodeRefAttribute, OnAttribute, Set};
use leptos::prelude::{RwSignal, StyleAttribute};
use leptos::task::spawn_local;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;

#[leptos::component]
pub fn LoginPage() -> impl leptos::IntoView {
    let cached_user_data_exists = CachedUser::read_state().user.is_some();
    let _cached_user_data = CachedUser::read_state().user;
    let (account_exists, set_account_exists) = signal(cached_user_data_exists);
    let open_loader = RwSignal::new(false);

    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());

    let email_input_element: NodeRef<html::Input> = NodeRef::new();
    let password_input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        open_loader.set(true);

        let email_binding = email_input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        let password_binding = password_input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        set_email.set(email_binding);
        set_password.set(password_binding);

        let login_form_data = LoginFormData::new(email.get(), password.get());

        spawn_local(async move {
            let login_response = login_form_data.submit().await;

            match login_response {
                Ok(FormResponse::<LoginResponse> { body, .. }) => {
                    println!("Login response: {:?}", body);

                    // fetch the user profile and state the state
                    let bearer_token = body.unwrap().token;
                    let user_profile = UserProfile::fetch(&bearer_token).await;
                    if let Err(error) = user_profile {
                        eprintln!("{}", error.to_string());
                        open_loader.set(false);
                        return;
                    }

                    let user_profile = user_profile.unwrap().body.unwrap();

                    CachedUser::set_user(user_profile);
                }
                Err(error) => {
                    open_loader.set(false);
                    println!("Login error: {:?}", error);
                }
            }
        });
    };

    view! {
        <div class="relative " style="height:calc(100vh - 100px)" id="editor">
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
            {if (move || account_exists.get())() {
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
            <form class="flex flex-col gap-y-4 mt-6" on:submit=on_submit>
                {if (move || !account_exists.get())() {
                    Either::Right(
                        view! {
                            <div class="form-input">
                                <label for="email">Email</label>
                                <input
                                    value=email
                                    node_ref=email_input_element
                                    type="email"
                                    placeholder="type your email"
                                />
                            </div>
                            <div class="form-input">
                                <label for="password">Password</label>
                                <input
                                    value=password
                                    node_ref=password_input_element
                                    type="password"
                                    placeholder="type your password"
                                />
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

                    type="submit"
                    class="btn w-full rounded-lg py-4 bg-app-600 text-white font-medium"
                >
                    Continue
                </button>
            </form>
            <a href="/dashboard" class="text-app block  text-sm font-bold mt-3">
                Forgotten password?
            </a>

        </div>

        <LoaderDots open_loader />
    }
}
