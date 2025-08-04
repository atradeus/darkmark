use crate::component::region::Region;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};

#[component]
pub fn HomePage() -> impl IntoView {
    let _app_name = t!("app_name");

    view! {
        <div class="w-40">
            <Region />
        </div>
    }
}
