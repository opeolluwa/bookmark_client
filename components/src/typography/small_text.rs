use leptos::either::Either;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{prelude::Children, view, IntoView};

#[leptos::component]
pub fn SmallText(
    #[prop(optional)] class: &'static str,
    #[prop(optional, default = "")] href: &'static str,
    children: Children,
) -> impl IntoView {
    {
        if href.is_empty() {
            Either::Right(
                view! { <p class=format!("leading-5 text-sm text-inherit {class}")>{children()}</p> },
            )
        } else {
            Either::Left(view! {
                <a href=href class=format!("leading-5 text-sm text-inherit {class}")>
                    {children()}
                </a>
            })
        }
    }
}
