use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::typography::heading::PageHeading;
use bookmark_components::typography::small_text::SmallText;
use bookmark_components::view::View;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;

#[leptos::component]
pub fn SearchPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! {
            <form class="grid grid-cols-12 items-center w-full py-0 my-0">
                <a href="/dashboard" class="col-span-1">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="size-5"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M17 10a.75.75 0 0 1-.75.75H5.612l4.158 3.96a.75.75 0 1 1-1.04 1.08l-5.5-5.25a.75.75 0 0 1 0-1.08l5.5-5.25a.75.75 0 1 1 1.04 1.08L5.612 9.25H16.25A.75.75 0 0 1 17 10Z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </a>
                <div class="col-span-10 bg-gray-100/40 rounded">
                    <input
                        type="text"
                        autofocus
                        placeholder="search"
                        class="input my-0 py-0 input-ghost input-sm w-full max-w-xs grow focus:border-none focus:outline-none  placeholder:dark:text-gray-700 placeholder:text-sm bg-transparent"
                    />
                </div>
                <button type="submit" class="mx-0 px-0 col-span-1">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="size-5"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z"
                            clip-rule="evenodd"
                        />
                    </svg>

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
