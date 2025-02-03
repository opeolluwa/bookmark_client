use leptos::prelude::*;

#[leptos::component]
pub fn AutoLoginComponent() -> impl leptos::IntoView {
    view! {
        <label class="swap swap-rotate">
            <input type="checkbox" class="toggle toggle-sm" checked="checked" />
        </label>
    }
}
