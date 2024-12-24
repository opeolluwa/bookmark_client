use bookmark_components::forms::sign_up::SignUpFormData;
use bookmark_components::forms::sign_up::SignUpResponse;
use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;
use leptos::either::Either;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::{NodeRef, NodeRefAttribute, OnAttribute, Set};
use leptos::task::spawn_local;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get},
    view,
};

use tauri_wasm_bindgen::command_hooks::SIGN_UP_COMMAND_HOOK;
use tauri_wasm_bindgen::core::invoke::invoke_command;

#[leptos::component]
pub fn SignUpPage() -> impl leptos::IntoView {
    let (is_loading, set_is_loading) = signal(false);
    let (first_name, set_first_name) = signal("".to_string());
    let (last_name, set_last_name) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());

    let first_name_input_element: NodeRef<html::Input> = NodeRef::new();
    let last_name_input_element: NodeRef<html::Input> = NodeRef::new();
    let email_input_element: NodeRef<html::Input> = NodeRef::new();
    let password_input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_is_loading.set(true);

        let first_name_binding = first_name_input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        let last_name_binding = last_name_input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        let email_binding = email_input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        let password_binding = password_input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        set_first_name.set(first_name_binding);
        set_last_name.set(last_name_binding);
        set_email.set(email_binding);
        set_password.set(password_binding);

        let sign_up_form_data = SignUpFormData::new(
            first_name.get(),
            last_name.get(),
            email.get(),
            password.get(),
        );

        spawn_local(async move {
            let command_response: Result<SignUpResponse, String> =
                invoke_command(SIGN_UP_COMMAND_HOOK, Some(sign_up_form_data)).await;

            println!("res   {:#?}", command_response);
            // let command_argument = serde_wasm_bindgen::to_value(&TauriCommandArgument {
            //     payload: sign_up_form_data,
            // })
            // .unwrap();

            // let result = tauri_sys::core::invoke::<TauriCommandResponse<SignUpResponse>>(
            //     SIGN_UP_COMMAND_HOOK,
            //     command_argument,
            // )
            // .await
            // .unwrap();

            // println!("{:#?}", result);

            // let result = invoke(SIGN_UP_COMMAND_HOOK, command_argument).await;
            // let api_response =
            //     serde_wasm_bindgen::from_value::<IpcResponseSuccess<SignUpResponse>>(result)
            //         .expect("error parsing APi response");
            // if api_response.status == IpcResponseStatus::Failed {
            //     //todo
            // };
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="">
            <div class="mb-12 flex justify-between items-center">
                <a href="/" class="block size-6">
                    <ArrowLongLeftIcon />
                </a>
            </div>
            <div class="mb-6">
                <Heading class="font-bold".into()>Create account</Heading>
                <SmallText class="leading-1 mt-2 text-sm small-text">
                    Create and account to begin
                </SmallText>
            </div>
            <form class="flex flex-col gap-y-4 mt-6" on:submit=on_submit>
                <div class="form-input">
                    <label for="first_name">First name</label>
                    <input
                        value=first_name
                        node_ref=first_name_input_element
                        type="text"
                        placeholder="type your first name"
                    />
                // <p>"Name is: " {first_name}</p>
                // <p>"Name is: " {last_name}</p>
                // <p>"Name is: " {email}</p>
                // <p>"Name is: " {password}</p>
                </div>

                <div class="form-input">
                    <label for="last_name">last name</label>
                    <input
                        value=last_name
                        node_ref=last_name_input_element
                        type="text"
                        placeholder="type your last name"
                    />
                </div>
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

                <button
                    disabled=is_loading
                    type="submit"
                    class="btn w-full disabled:bg-100 rounded-lg py-4 bg-app-600 text-white font-medium"
                >
                    {if is_loading.get() == true {
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
