use chrono::Local;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_heroicons::size_24::outline::Star;

use crate::typography::small_text::SmallText;
use crate::view::View;

#[leptos::component]
pub fn BookmarkCard(
    title: &'static str,
    #[prop(optional)] description: &'static str,
    date: &'static str,
) -> impl leptos::IntoView {
    view! {
        <View class="flex gap-x-3 justify-between bg-white rounded-lg shadow-sm border-gray-500 w-full px-3 py-6">
            <div class="flex items-center gap-x-3 align-center">
                <button class="text-gray-400 size-5">
                    <Star />
                </button>
                <div>
                    <h3 class="font-medium w-[4/5] text-gray-600/80 truncate">{title}</h3>
                    <SmallText class="leading-1 text-gray-400">{description}</SmallText>
                </div>
            </div>
            <div>
                <SmallText class="text-[9.5px] text-gray-500">{date}</SmallText>
            </div>
        </View>
    }
}
