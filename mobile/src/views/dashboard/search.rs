use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_heroicons::size_24::solid::{ChevronLeft, MagnifyingGlass as SearchIcon};

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;
use bookmark_components::typography::small_text::SmallText;
use bookmark_components::view::View;

#[leptos::component]
pub fn SearchPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! {
            <form class="flex gap-x-1 justify-evenly -mt-[3px]  items-center">
                <a href="/dashboard" class="col-span-1 size-5 font-bold text-black">
                    <ChevronLeft class="" />
                </a>
                <input
                    type="text"
                    autofocus
                    placeholder="search"
                    class="input input-sm flex my-0 py-0 focus:border-none focus:outline-none  placeholder:dark:text-gray-700 w-[80vw] border rounded"
                />
                <button type="submit" class="col-span-1 size-5 font-bold text-black">
                    <SearchIcon  />
                </button>
            </form>
        }>

            <View class="flex flex-col  h-[85vh] items-center justify-center">
                <PageHeading text="No result" />
                <SmallText class="mt-1 text-gray-400">try another keyword</SmallText>
            </View>
        </DashboardLayout>
    }
}
