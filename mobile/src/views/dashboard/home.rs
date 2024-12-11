use crate::layout::dashboard_layout::DashboardLayout;
use bookmark_components::icons::plus_icon::PlusIcon;
use bookmark_components::layout::view::View;
use bookmark_components::typography::heading::PageHeading;
use leptos::prelude::CustomAttribute;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::prelude::{OnAttribute, RwSignal, Set};
use leptos::view;
use leptos_heroicons::size_24::outline::PencilSquare;
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
            <div class="flex justify-between items-center">
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
                <PageHeading text="Home" />
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
                <DrawerHeaderTitle>
                    <PageHeading text="Collections" />
                </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody class="w-[60vw] relative">
                <button class="btn border-app bg-transparent w-full absolute bottom-3 left-0 right-0">
                    new collection <PlusIcon />
                </button>
            </DrawerBody>
        </OverlayDrawer>
    }
}

#[leptos::component]
pub fn HomePageHeader() -> impl leptos::IntoView {
    view! {}
}
