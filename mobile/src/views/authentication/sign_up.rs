use bookmark_components::forms::sign_up::RegisterFormData;
use bookmark_components::icons::arrow_left_right_icon::ArrowLongLeftIcon;
use bookmark_components::js_bindings::navigate::change_location_to;
use bookmark_components::loaders::loader_dots::LoaderDots;
use bookmark_components::typography::heading::Heading;
use bookmark_components::typography::small_text::SmallText;
use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::{NodeRef, NodeRefAttribute, OnAttribute, Set};
use leptos::task::spawn_local;
use leptos::{
    prelude::{signal, ClassAttribute, ElementChild, Get, RwSignal},
    view,
};
use leptos_router::hooks::use_navigate;

#[leptos::component]
pub fn SignUpPage() -> impl leptos::IntoView {
    let open_loader = RwSignal::new(false);

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
        open_loader.set(true);

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

        let sign_up_form_data = RegisterFormData::new(
            first_name.get(),
            last_name.get(),
            email.get(),
            password.get(),
        );

        spawn_local(async move {
            let sign_up_response = sign_up_form_data.submit().await;

            match sign_up_response {
                Ok(_) => {
                    //TODO: automatically login
                    change_location_to("/dashboard");
                }
                Err(error) => {
                    log::error!("Failed to sign up due to error {}", error.to_string());
                    return;
                }
            }
        });
        open_loader.set(false);
    };

    view! {
        <div id="editor"></div>
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
            <form class="flex flex-col gap-y-4 mt-6" on:submit=on_submit>
                <div class="form-input">
                    <label for="first_name">First name</label>
                    <input
                        value=first_name
                        node_ref=first_name_input_element
                        type="text"
                        placeholder="type your first name"
                    />

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
                    type="submit"
                    class="btn w-full disabled:bg-100 rounded-lg py-4 bg-app-600 text-white font-medium"
                >
                    Continue

                </button>
            </form>
            <a href="/forgotten-password" class="text-app block  text-sm font-bold mt-3">
                "Already have and account? Login"
            </a>

        </div>

        <LoaderDots open_loader />
    }
}

fn navigate() {
    let navigate = use_navigate();
    navigate("/dashboard", Default::default());
}
