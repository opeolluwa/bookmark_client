use leptos::{
    prelude::{ClassAttribute, RwSignal},
    view,
};
use thaw::{Dialog, DialogBody, DialogSurface};

#[leptos::component]
pub fn LoaderDots(open_loader: RwSignal<bool>) -> impl leptos::IntoView {
    view! {
        <Dialog open=open_loader mask_closeable=false close_on_esc=false>
            <DialogSurface class="bg-transparent flex items-center justify-center">
                <DialogBody>
                    <div class="loader-dots"></div>

                </DialogBody>
            </DialogSurface>
        </Dialog>
    }
}
