use crate::typography::small_text::SmallText;
use crate::view::View;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_heroicons::size_24::solid::Star;

#[leptos::component]
pub fn BookmarkCard(
    title: &'static str,
    description: &'static str,
    date: &'static str,
) -> impl leptos::IntoView {
    view! {
        <View class="flex flex-col bg-white rounded-md border-gray-400 w-full px-3 py-6">
            <div class="flex justify-between items-center">
              <Star class="size-5 text-gray-400"/>
                <h3 class="text-bold">{title}</h3>
                <SmallText>{date}</SmallText>
            </div>
            <p class="mt-2 text-gray-400">{description}</p>
        </View>
    }
}
