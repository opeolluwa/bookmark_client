use leptos::view;

use bookmark_components::layouts::mobile::dashboard::DashboardLayout;
use bookmark_components::notifications::notification_card::NotificationCard;
use bookmark_components::typography::heading::PageHeading;
use bookmark_components::view::View;

#[leptos::component]
pub fn NotificationsPage() -> impl leptos::IntoView {
    view! {
        <DashboardLayout header_component=view! { <PageHeading text="Notification" /> }>
            <View class="flex flex-col gap-y-1">
                <NotificationCard />
                <NotificationCard />
                <NotificationCard />
                <NotificationCard />

            </View>
        </DashboardLayout>
    }
}
