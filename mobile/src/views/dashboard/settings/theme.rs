use leptos::prelude::*;
use leptos::view;
use thaw::Icon;

#[leptos::component]
pub fn ColorThemeSwapComponent() -> impl leptos::IntoView {
    view! {
        <label class="swap swap-rotate">
            <input type="checkbox" class="theme-controller" value="synthwave" />
            <Icon icon=icondata::BsSun class="swap-off size-5 fill-current" />
            <Icon icon=icondata::BsMoon class="swap-on size-5 fill-current" />
        </label>
    }
}
