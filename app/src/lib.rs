#[macro_use]
extern crate rust_i18n;
pub mod api;
pub mod component;
pub mod page;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
// use leptos_use::{use_color_mode_with_options, UseColorModeOptions, UseColorModeReturn};
use crate::page::login::Login;
use page::home::HomePage;
use rust_i18n::t;

i18n!("locales", fallback = "en");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    // let UseColorModeReturn { mode, set_mode, .. } =
    //     use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));
    // let UseColorModeReturn { mode, .. } =
    //     use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

    view! {
        <!DOCTYPE html>
        // <html lang="en" class=move || mode.get().to_string()>
        <html lang="en" data-theme="black">
            // <html lang="en" class="light">
            <base href="." />
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                // <base href=".">
                <leptos_meta::MetaTags />
                // <meta name="color-scheme" content="dark light" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <link rel="stylesheet" id="leptos" href="/pkg/darkmark.css" />
            </head>
            <body class="not-prose relative left-1/2 w-dvw max-w-none -translate-x-1/2">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    view! {
        <Title text=t!("app_name").to_string() />
        <div class="container gap-4 grid grid-cols-1 mx-auto text-center bg-stone-900 text-white">
            <Router>
                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("/") view=HomePage />
                        <Route path=StaticSegment("/login") view=Login />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
