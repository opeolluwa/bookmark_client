use bookmark_components::cards::bookmark::BookmarkCard;
use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;
use leptos::prelude::OnAttribute;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos_heroicons::size_24::outline::Plus as PlusIcon;
use leptos_heroicons::size_24::solid::MagnifyingGlass as SearchIcon;

#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    let navigate = leptos_router::hooks::use_navigate();

    view! {
        <DashboardLayout header_component=view! {
            <div class="flex justify-between items-center w-full">
                <PageHeading text="Bookmarks" />

                <button on:click=move |_| { navigate("/dashboard/search", Default::default()) }>
                    <SearchIcon class="size-5 font-bold text-black" />
                </button>

            </div>
        }>

            <div>

                <BookmarkCard
                    title="The dangers of doing as you like"
                    date="July 5, 2023"
                    description="all that is good is also all good"
                />
            // <BookmarkCard
            // title="The dangers of doing as you like"
            // date="July 5, 2023"
            // description="all that is good is also all good"
            // />
            // <BookmarkCard
            // title="The dangers of doing as you like"
            // date="July 5, 2023"
            // description="all that is good is also all good"
            // />
            </div>
        </DashboardLayout>

        <a
            href="/editor"
            class=" size-10 p-2 border-transparent text-white dark:text-gray-400 bg-app shadow fixed  rounded-full bottom-20 right-4"
        >
            <PlusIcon />
        </a>
    }
}
