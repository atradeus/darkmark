use leptos::prelude::{
    ClassAttribute, CollectView, ElementChild, Get, GlobalAttributes, OnTargetAttribute,
    PropAttribute, RenderHtml, RwSignal, Set,
};
use leptos::{component, view, IntoView};
use lib::SelectOpt;
use leptos::prelude::IntoMaybeErased;

#[component]
pub fn Select(
    id: &'static str,
    label: &'static str,
    options: Option<Vec<SelectOpt>>,
    value: RwSignal<String>,
    #[prop(optional)] opt_label: Option<&'static str>,
    #[prop(optional)] _error: RwSignal<bool>,
) -> impl IntoView {
    let mut options = options.unwrap_or_else(|| vec![]);
    
    if opt_label.is_some() {
        options.insert(
            0,
            SelectOpt::new(String::new(), opt_label.unwrap().into_owned()),
        );
    }

    view! {
        <div class="max-w-2xl mx-auto">
            <label for=id class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-400">
                {label}
            </label>
            <select
                id=id
                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                on:change:target=move |ev| {
                    let _v: String = ev.target().value().parse().unwrap();
                    value.set(ev.target().value().parse().unwrap());
                }
                prop:value=move || value.get().to_string()
            >
                {options
                    .into_iter()
                    .map(|opt| {
                        view! { <option value=opt.value>{opt.label}</option> }
                    })
                    .collect_view()}
            </select>
        </div>
    }
}
