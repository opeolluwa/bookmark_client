use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use thaw::Avatar;

use crate::typography::small_text::SmallText;
use crate::view::View;

#[leptos::component]
pub fn NotificationCard() -> impl leptos::IntoView {
    view! {
        <View class="card_layout">
            <div class="grid grid-cols-12 items-center gap-x-3 align-center">
                <Avatar size=45 src="assets/img/default-user.png" class="col-span-2" />

                <div class="col-span-9 ">
                    <h3 class="card_title">Title goes here</h3>
                    <SmallText class="leading-1 text-gray-400 line-clamp-2">
                        Cupidatat officia enim laborum aute quis sunt quis cillum mollit consequat esse nostrud exercitation.
                    </SmallText>
                </div>
            </div>

            <div></div>
        </View>
    }
}
