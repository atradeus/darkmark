use leptos::prelude::{
    ClassAttribute, CollectView, ElementChild, Get, GlobalAttributes, OnTargetAttribute,
    PropAttribute, RenderHtml, RwSignal, Set,
};
use leptos::{component, view, IntoView};
use lib::SelectOpt;

#[component]
pub fn Select(
    id: &'static str,
    label: &'static str,
    options: Option<Vec<SelectOpt>>,
    value: RwSignal<String>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] opt_label: Option<&'static str>,
    #[prop(optional)] error: Option<bool>,
) -> impl IntoView {
    let mut options = options.unwrap_or_else(|| vec![]);
    let disabled = disabled.unwrap_or(false);
    
    if opt_label.is_some() {
        options.insert(
            0,
            SelectOpt::new(String::new(), opt_label.unwrap().into_owned(), true),
        );
    }

    let mut class = "select";
    if error.is_some() && error.unwrap() {
        class = "select select-error";
    }

    view! {
        <div class="mx-auto">
            <label for=id class="block text-sm font-medium mb-3">
                {label}
            </label>
            <select
                id=id
                disabled=disabled
                class=class
                on:change:target=move |ev| {
                    let _v: String = ev.target().value().parse().unwrap();
                    value.set(ev.target().value().parse().unwrap());
                }
                prop:value=move || value.get().to_string()
            >
                {options
                    .into_iter()
                    .map(|opt| {
                        view! {
                            <option value=opt.value disabled=opt.disabled>
                                {opt.label}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
        </div>
    }
}
