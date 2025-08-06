use crate::component::region::Region;
use leptos::prelude::ElementChild;
use leptos::{component, view, IntoView};
use reactive_graph::signal::RwSignal;
use crate::component::navbar::Navbar;

#[component]
pub fn HomePage() -> impl IntoView {
    let _app_name = t!("app_name");
    let region = RwSignal::new(String::from(""));

    view! {
        <Navbar />
        <Region region=region />
        <div>value: {region}</div>
    }
}
