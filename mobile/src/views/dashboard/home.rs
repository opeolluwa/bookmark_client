use bookmark_hooks::logout::logout_current_user;
use leptos::prelude::{
    signal, ClassAttribute, CollectView, ElementChild, Get, OnAttribute, Set,
};
use leptos::prelude::{RwSignal, Show};
use leptos::view;
use leptos_heroicons::size_24::outline::Plus as PlusIcon;
use thaw::{DrawerBody, DrawerPosition, Flex, FlexAlign, FlexJustify, Icon, Image, OverlayDrawer};

use bookmark_components::cards::bookmark::{BookmarkCardProps, BookmarkSectionTitle};
use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;
use bookmark_components::typography::small_text::SmallText;

#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    let open = RwSignal::new(false);
    let bookmarks: Vec<BookmarkCardProps> = vec![];
    let (collections, _set_collections) = signal::<Vec<String>>(vec![]);

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

            <Show
                when=move || !bookmarks.is_empty()
                fallback=|| {
                    view! {
                        <div class="flex flex-col items-center justify-center h-[70vh] text-center">
                            <Image
                                class="w-1/5 -mb-3"
                                src="/assets/illustrations/empty-box.png"
                                alt="empty"
                            />

                            <BookmarkSectionTitle title="Whoo-hoo!" />
                            <SmallText>No Bookmarks added yet</SmallText>
                        </div>
                    }
                }
            >

                hey
            </Show>

        </DashboardLayout>

        <a href="/editor" class="fab">
            <PlusIcon />
        </a>

        <OverlayDrawer open position=DrawerPosition::Left>
            <DrawerBody class="relative">
                <Flex align=FlexAlign::Center justify=FlexJustify::SpaceBetween>
                    <PageHeading text="Collections" />
                    <button on:click=move |_| logout_current_user()>

                        <Icon icon=icondata::TbLogout2 class="size-5" />
                    </button>

                </Flex>

                <Show
                    when=move || !collections.get().is_empty()
                    fallback=|| {
                        view! {
                            <a
                                href="/collection"
                                class="btn bg-app text-white absolute bottom-2 left-0 right-0 w-full shadow mt-auto"
                            >
                                Create Collection
                                <Icon icon=icondata::AiPlusOutlined class="size-5" />
                            </a>
                        }
                    }
                >

                    <ul>
                        {collections
                            .get()
                            .into_iter()
                            .map(|entry| view! { <li>{entry}</li> })
                            .collect_view()}
                    </ul>
                </Show>

            </DrawerBody>
        </OverlayDrawer>
    }
}
