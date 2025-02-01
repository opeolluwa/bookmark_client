use bookmark_components::forms::login::LoginFormData;
use bookmark_state::cached_user::CachedUser;
use leptos::either::Either;
use leptos::html;
use leptos::prelude::{NodeRef, NodeRefAttribute, OnAttribute, Set};
use leptos::task::spawn_local;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use bookmark_components::forms::login::LoginResponse;
use bookmark_components::typography::heading::Heading;
use leptos::ev::SubmitEvent;

use bookmark_components::forms::user_profile::UserProfile;
use bookmark_components::forms::FormResponse;
use leptos::prelude::RwSignal;

#[leptos::component]
pub fn LoginPage() -> impl leptos::IntoView {
    let (is_loading, _set_is_loading) = signal(false);

    let cached_user_data_exists = CachedUser::read_state().user.is_some();
    let cached_user_data = CachedUser::read_state().user.unwrap();
    let (account_exists, _) = signal(cached_user_data_exists);

    let open_loader = RwSignal::new(false);
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());

    let email_input_element: NodeRef<html::Input> = NodeRef::new();
    let password_input_element: NodeRef<html::Input> = NodeRef::new();

    let cached_emil = cached_user_data.email;
    let cached_first_name = cached_user_data.first_name;

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

                    println!("User profile: {:?}", user_profile);
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
        <div class="h-screen flex justify-center items-center flex-col gap-x-12 bg-gray-50 absolute w-full bg-[url(/assets/background/topography.svg)] bg-no-repeat bg-gray-040 bg-cover bg-blend-multiply">
            <div class="pb-6 py-2 px-6 w-[40%] bg-white rounded-lg shadow-lg">
                    <div class="mt-4 mb-2 text-center">
                        {if (move || account_exists.get())() {
                            Either::Right(
                                view! {
                                    <Heading class="font-bold">Welcome, {cached_first_name}</Heading>

                                },
                            )
                        } else {
                            Either::Left(
                                view! {
                                    <Heading class="font-bold">Log in</Heading>

                                },
                            )
                        }}
                    </div>

                    <form class="flex flex-col gap-y-4 mt-6" on:submit=on_submit>

                        {if (move || !account_exists.get())() {
                            Either::Right(
                                view! {
                                    <div class="form-input desktop">
                                        <label for="email">Email</label>
                                        <input
                                            value=email
                                            node_ref=email_input_element
                                            type="email"
                                            placeholder="type your email"
                                        />
                                    </div>
                                    <div class="form-input desktop">
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
                            class="btn border-none hover:bg-app-600/95 w-full rounded-lg py-4 bg-app-600 text-white font-medium"
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
                    <a
                        href="/dashboard"
                        class="text-gray-600/50 block text-sm font-medium mt-3"
                    >
                        Forgotten password?
                    </a>

            </div>
        </div>
    }
}
