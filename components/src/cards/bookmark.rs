use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use thaw::Icon;

use crate::typography::small_text::SmallText;
use crate::view::View;

#[leptos::component]
pub fn BookmarkCard(
    title: &'static str,
    #[prop(optional)] description: &'static str,
    date: &'static str,
) -> impl leptos::IntoView {
    view! {
        <View class="card_layout">
            <div class="grid grid-cols-12 items-center gap-x-3 align-center">
                <Icon icon=icondata::HiStarOutlineLg class="text-gray-400 col-span-1 size-4" />
                <div class="col-span-11">
                    <h3 class="card_title">{title}</h3>
                    <SmallText class="leading-1 text-gray-400 line-clamp-2">
                        {description}
                    </SmallText>
                </div>
            </div>
            <div>
                <SmallText class="text-[9.5px] text-gray-500">{date}</SmallText>
            </div>
        </View>
    }
}
