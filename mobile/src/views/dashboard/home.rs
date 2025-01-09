use leptos::prelude::{ClassAttribute, ElementChild, OnAttribute, Set};
use leptos::prelude::{CollectView, RwSignal};
use leptos::view;
use leptos_heroicons::size_24::outline::Plus as PlusIcon;
use thaw::{DrawerBody, DrawerPosition, Icon, OverlayDrawer};

use bookmark_components::cards::bookmark::{BookmarkCard, BookmarkCardProps};
use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;

#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    let open = RwSignal::new(false);
    let bookmark_one: Vec<BookmarkCardProps> = vec![
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Voluptate ea Lorem Anim id excepteur elit ad aliqua aliqua. Culpa proident magna magna aliqua sint ipsum. cupidatat mollit excepteur pariatur nulla.",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Minim non fugiat magna incididunt ex eu dolor esse velit.",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Lorem voluptate elit deserunt laboris nulla eu ullamco.",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Pariatur proident officia commodo laborum nostrud.",
        },
    ];

    let bookmark_two: Vec<BookmarkCardProps> = vec![
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Lorem reprehenderit ut culpa do qui dolore cupidatat culpa ullamco minim ea sit proident.",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Id ipsum excepteur aliqua proident irure labore laboris incididunt velit non excepteur ea esse fugiat velit.",
        },
        BookmarkCardProps {
            title: "some title goes here",
            date: "2d",
            description: "Deserunt consequat velit sint consectetur nostrud pariatur laborum sunt velit sint minim incididunt.",
        },
    ];

    view! {
        <DashboardLayout header_component=view! {
            <div class="flex justify-between items-center w-full">
                <div class="flex  items-center gap-x-2">
                    <button on:click=move |_| open.set(true)>
                        <Icon icon=icondata::TbMenu2 class="size-5" />
                    </button>
                    <PageHeading text="Bookmarks" />
                </div>

                <a href="/dashboard/search">
                    <Icon icon=icondata::LuSearch class="size-5" />

                </a>
            </div>
        }>

            <h2 class="text-[18px] leading-2 text-gray-600/90 font-medium mb-2">This week</h2>
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

            <h2 class="text-[18px] leading-2 text-gray-600/90 font-medium mb-2 mt-12">Last week</h2>
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

        <a href="/editor" class="fab">
            <PlusIcon />
        </a>

        <OverlayDrawer open position=DrawerPosition::Left>
            <DrawerBody>
                <p>"Drawer content"</p>
            </DrawerBody>
        </OverlayDrawer>
    }
}
