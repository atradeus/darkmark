use crate::api::{fetch_api};
use crate::component::select::Select;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use lib::SelectOpt;

pub async fn fetch_regions() -> Vec<SelectOpt> {
    let url = format!("{}/region", crate::api::get_site_addr());
    match fetch_api::<Vec<lib::Region>>(&url).await {
        Some(regions) => regions
            .into_iter()
            .map(|r| SelectOpt::new(r.code, r.name,false))
            .collect::<Vec<_>>(),
        None => Vec::new(),
    }
}

#[component]
pub fn Region(region: RwSignal<String>) -> impl IntoView {

    let regions = LocalResource::new(|| async move {
        fetch_regions().await
    });

    view! {
        <Suspense fallback=|| {
            view! { <p id="loading">"Loading..."</p> }
        }>
            {move || Suspend::new(async move {
                let items = regions.await;
                view! {
                    <div class="w-40">
                        <Select
                            id="regions"
                            label="Regions"
                            value=region
                            options=Some(items)
                            opt_label="Select Region"
                        />
                    </div>
                }
            })}
        </Suspense>
    }
    .into_any()
}
