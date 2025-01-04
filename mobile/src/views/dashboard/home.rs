use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::prelude::{CollectView, OnAttribute};
use leptos::view;
use leptos_heroicons::size_24::outline::Plus as PlusIcon;
use leptos_heroicons::size_24::solid::MagnifyingGlass as SearchIcon;

use bookmark_components::cards::bookmark::{BookmarkCard, BookmarkCardProps};
use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;

#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    let navigate = leptos_router::hooks::use_navigate();

    let bookmark_one: Vec<BookmarkCardProps> = vec![
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "adfkln.a fd;kj",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "m,d;'ejdn",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: ";jladnf l",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "q;jorqn",
        },
    ];

    let bookmark_two: Vec<BookmarkCardProps> = vec![
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "m,d;'ejdn",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: ";jladnf l",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "q;jorqn",
        },
    ];

    view! {
        <DashboardLayout header_component=view! {
            <div class="flex justify-between items-center w-full">
                <PageHeading text="Bookmarks" />

                <button on:click=move |_| { navigate("/dashboard/search", Default::default()) }>
                    <SearchIcon class="size-5 font-bold text-black" />
                </button>

            </div>
        }>

                 <h2 class="text-[18px] leading-2 text-gray-600/90 font-medium mb-2 mt-12">
               This week
            </h2>
            <div class="flex flex-col gap-y-1">
                {bookmark_one
                    .into_iter()
                    .map(|entry| {
                        view! {
                            <BookmarkCard
                                title=entry.title
                                date=entry.date
                                description=entry.description
                            />
                        }
                    })
                    .collect_view()}

            </div>

            <h2 class="text-[18px] leading-2 text-gray-600/90 font-medium mb-2 mt-12">
                Last week
            </h2>
            <div class="flex flex-col gap-y-1">
                {bookmark_two
                    .into_iter()
                    .map(|entry| {
                        view! {
                            <BookmarkCard
                                title=entry.title
                                date=entry.date
                                description=entry.description
                            />
                        }
                    })
                    .collect_view()}

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
