use crate::typography::small_text::SmallText;
use crate::view::View;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_heroicons::size_24::outline::Star;
use leptos_heroicons::size_24::solid::EllipsisVertical;

#[leptos::component]
pub fn BookmarkCard(
    title: &'static str,
    description: &'static str,
    date: &'static str,
) -> impl leptos::IntoView {
    view! {
        <View class="flex gap-x-3 items-center  bg-white rounded-md border-gray-400 w-full px-3 py-6">
            <button>
                <Star class="size-5 text-gray-400" />
            </button>
            <div class="flex flex-col justify-between items-center">
                <div class="flex">
                    <h3 class="text-bold">{title}</h3>
                    <SmallText>{date}</SmallText>
                </div>
                <p class="mt-2 text-gray-400">{description}</p>
            </div>
            <button>
                <EllipsisVertical class="size-5 text-gray-400" />
            </button>
        </View>
    }
}
