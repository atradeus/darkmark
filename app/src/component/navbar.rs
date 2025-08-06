use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
	let app_name = t!("app_name");
	view! {
		<div class="navbar bg-primary text-primary-content mb-10">
			<div class="navbar-start">
				<div class="text-xl">{app_name}</div>
			</div>
			<div class="navbar-end">
				<a href="/" class="btn btn-ghost">
					Home
				</a>
			</div>
		</div>
	}
}