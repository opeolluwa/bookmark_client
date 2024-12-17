use crate::layout::dashboard_layout::DashboardLayout;
use bookmark_components::icons::plus_icon::PlusIcon;
use bookmark_components::typography::heading::PageHeading;
use bookmark_components::typography::small_text::SmallText;
use leptos::prelude::CustomAttribute;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::prelude::{OnAttribute, RwSignal, Set};
use leptos::view;
use thaw::{
    Avatar, DrawerBody, DrawerHeader, DrawerHeaderTitle, Menu, MenuItem, MenuTrigger,
    MenuTriggerType, OverlayDrawer,
};

#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    let open = RwSignal::new(false);
    let on_select = move |key: String| {
        leptos::logging::warn!("{}", key);
    };

    view! {
        <DashboardLayout header_component=view! {
            <div class="flex justify-between items-center w-full">
                <button on:click=move |_| open.set(true)>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="size-5"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M2 4.75A.75.75 0 0 1 2.75 4h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 4.75ZM2 10a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75A.75.75 0 0 1 2 10Zm0 5.25a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H2.75a.75.75 0 0 1-.75-.75Z"
                            clip-rule="evenodd"
                        />
                    </svg>

                </button>
                <SmallText>Default Collection</SmallText>
                <Menu on_select trigger_type=MenuTriggerType::Hover>
                    <MenuTrigger slot>
                        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" />
                    </MenuTrigger>
                    <MenuItem value="facebook">"Logout"</MenuItem>
                    <MenuItem value="facebook">"Profile"</MenuItem>
                </Menu>
            </div>
        }>""</DashboardLayout>

        <OverlayDrawer open>
            <DrawerHeader>
                <DrawerHeaderTitle class="border-b flex w-full justify-between">
                    <PageHeading text="Collections" />
                    <button>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 20 20"
                            fill="currentColor"
                            class="size-5 hidden"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M3 4.25A2.25 2.25 0 0 1 5.25 2h5.5A2.25 2.25 0 0 1 13 4.25v2a.75.75 0 0 1-1.5 0v-2a.75.75 0 0 0-.75-.75h-5.5a.75.75 0 0 0-.75.75v11.5c0 .414.336.75.75.75h5.5a.75.75 0 0 0 .75-.75v-2a.75.75 0 0 1 1.5 0v2A2.25 2.25 0 0 1 10.75 18h-5.5A2.25 2.25 0 0 1 3 15.75V4.25Z"
                                clip-rule="evenodd"
                            />
                            <path
                                fill-rule="evenodd"
                                d="M19 10a.75.75 0 0 0-.75-.75H8.704l1.048-.943a.75.75 0 1 0-1.004-1.114l-2.5 2.25a.75.75 0 0 0 0 1.114l2.5 2.25a.75.75 0 1 0 1.004-1.114l-1.048-.943h9.546A.75.75 0 0 0 19 10Z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </button>
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody class="relative">
                <div class="w-full border-t absolute bottom-2 left-0 right-0">
                    <button class="btn w-full flex border-app bg-transparent">
                        New collection <PlusIcon />
                    </button>

                </div>
            </DrawerBody>
        </OverlayDrawer>
    }
}

#[leptos::component]
pub fn HomePageHeader() -> impl leptos::IntoView {
    view! {}
}
