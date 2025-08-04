use crate::api::fetch_regions;
use crate::component::select::Select;
use leptos::prelude::*;
use leptos::{component, view, IntoView};



#[component]
pub fn Region() -> impl IntoView {

    let region = RwSignal::new(String::from(""));

    let regions = OnceResource::new(fetch_regions());

    view! {
        <div class="w-40">
            <Select
                id="regions"
                label="Regions"
                // value=region options={async move  || {regions.await}}
                value=region
                options=regions.get()
            />
            // opt_label=Some("Select Region")
            <div>{"Value: "}{region}</div>

        </div>
    }
}
