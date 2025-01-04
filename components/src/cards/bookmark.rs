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
        <View class="flex gap-x-3 justify-between bg-white rounded border-gray-500 w-full px-3 py-6">
            <div class="flex gap-x-3 align-center">
                <button>
                    <Star class="size-5 text-gray-400" />
                </button>
                <div>
                    <h3 class="font-medium w-[4/5] text-gray-600/80 truncate">{title}</h3>
                    <SmallText class="leading-1 text-gray-400">{description}</SmallText>
                </div>
            </div>
            <div>
                <SmallText class="text-[10px]">{date}</SmallText>
            </div>
        </View>
    }
}

fn parse_time(date: &str) -> String {
    let current_time = Local::now();
    // let target_date = Local::from(date);

    todo!()
}
