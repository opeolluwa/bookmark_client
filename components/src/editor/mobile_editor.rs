use leptos::prelude::OnTargetAttribute;
use leptos::prelude::{
    signal, ElementChild, Get, GlobalAttributes, PropAttribute, Set, StyleAttribute,
};
use leptos::{prelude::ClassAttribute, view};
use leptos_heroicons::size_20::solid::{
    Bars3 as AlignCenter, Bars3BottomLeft as AlignLeft, Bars3BottomRight as AlignRight, Bold,
    CodeBracket, Italic, Link, ListBullet, NumberedList, Photo, Underline, H1, H2, H3,
};

use crate::editor::control_elements::control_btn::EditorControlButton;
use crate::editor::control_hooks::token::MarkdownToken;

#[leptos::component]
pub fn MarkdownEditor() -> impl leptos::IntoView {
    let bold_icon = view! { <Bold /> };
    let italic_icon = view! { <Italic /> };
    let underline_icon = view! { <Underline /> };
    let link_icon = view! { <Link /> };
    let list_bullet_icon = view! { <ListBullet /> };
    let numbered_list_icon = view! { <NumberedList /> };
    let h1_icon = view! { <H1 /> };
    let h2_icon = view! { <H2 /> };
    let h3_icon = view! { <H3 /> };
    let code_icon = view! { <CodeBracket /> };
    let image_icon = view! {<Photo/>};

    let (content, set_content) = signal("".to_string());

    view! {
        <div class="relative">
            <div id="output" class="z-50 hidden">
                {markdown::to_html(&content.get())}
            </div>

            <textarea
                name="bookmark_content"
                id="content"
                on:input:target=move |ev| {
                    set_content.set(ev.target().value());
                }

                autofocus
                class="bg-transparent w-full relative top-2 outline-none border-none placeholder:font-italic"
                placeholder="type here..."
                prop:value=content
            ></textarea>

            <div class="flex fixed bottom-0 left-0  z-50 right-0 px-4 btm-nav text-black overflow-x-scroll w-full bg-white p-2">
                <EditorControlButton icon=bold_icon token=MarkdownToken::Bold />
                <EditorControlButton icon=italic_icon token=MarkdownToken::Italic />
                <EditorControlButton icon=underline_icon token=MarkdownToken::Underline />
                <EditorControlButton icon=link_icon token=MarkdownToken::Link />
                <EditorControlButton icon=code_icon token=MarkdownToken::Code />
                <EditorControlButton icon=image_icon token=MarkdownToken::Image />
                <EditorControlButton icon=list_bullet_icon token=MarkdownToken::ListBullet />
                <EditorControlButton icon=numbered_list_icon token=MarkdownToken::NumberedList />
                <EditorControlButton icon=h1_icon token=MarkdownToken::H1 />
                <EditorControlButton icon=h2_icon token=MarkdownToken::H2 />
                <EditorControlButton icon=h3_icon token=MarkdownToken::H3 />
            </div>
        </div>
    }
}
