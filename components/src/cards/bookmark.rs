use crate::typography::small_text::SmallText;
use crate::view::View;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn BookmarkCard(
    title: &'static str,
    description: &'static str,
    date: &'static str,
) -> impl leptos::IntoView {
    view! {
        <View class="flex bg-white rounded w-full p-2">
            <div class="flex justify-between items-center">
                <h6>{title}</h6>
                <SmallText>{date}</SmallText>
            </div>
            <p class="mt-2 text-gray-400">{description}</p>
        </View>
    }
}
