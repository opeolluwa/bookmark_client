use leptos::prelude::ElementChild;
use leptos::{view, IntoView};

use crate::editor::control_hooks::token::MarkdownToken;
use crate::icon::HeroIcon;

#[leptos::component]
pub fn EditorControlButton<I>(icon: I, token: MarkdownToken) -> impl leptos::IntoView
where
    I: IntoView,
{
    // let token =
    println!("{}", token.to_string());
    view! {
        <button>
            <HeroIcon class="text-black" icon_data=icon />
        </button>
    }
}
