use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, Serialize, Deserialize, Default, EnumIter)]
pub enum Language {
    #[default]
    English,
    French,
    Spanish,
}

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::English => "english".to_string(),
            Language::French => "french".to_string(),
            Language::Spanish => "spanish".to_string(),
        }
    }
    pub fn collect() -> Vec<(String, String)> {
        Self::iter()
            .map(|entry| (entry.to_string(), entry.to_string()))
            .collect()
    }
}

#[component]
pub fn SettingsRoute<F>(
    /// The annotated text
    label: &'static str,
    /// (optional ) allow routes to be used as
    #[prop(optional)]
    href: Option<&'static str>,
    /// the icon
    icon: F,
) -> impl IntoView
where
    F: IntoView,
{
    let label = label.to_string();
    let _href = href.unwrap_or("#").to_string();

    view! { <div class="flex items-center gap-x-2">{icon} {label}</div> }
}

#[leptos::component]
pub fn SettingsTab<V>(
    /// the table title
    title: &'static str,
    /// (optional) class class to apply to the root element
    #[prop(optional)]
    class: &'static str,
    /// (optional) class to apply to the inner class
    #[prop(optional)]
    component_class: &'static str,
    /// the child
    component: V,
) -> impl leptos::IntoView
where
    V: IntoView,
{
    view! {
        <div class=format!("my-12 first:mt-0 last:mb-4 {class}")>
            <h2 class="text-sm font-medium capitalize">{title}</h2>
            <div class=format!("my-2 {component_class}")>{component}</div>
        </div>
    }
}
