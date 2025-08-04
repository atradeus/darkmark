use leptos::logging;
use leptos::prelude::get_configuration;
use reactive_graph::owner::on_cleanup;
use send_wrapper::SendWrapper;
use thiserror::Error;
use lib::{Region, SelectOpt};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod auth;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)] 
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(lib::Error),
    #[error("Login error")]
    LoginError(String),
}

impl From<lib::Error> for Error {
    fn from(e: lib::Error) -> Self {
        Self::Api(e)
    }
}

fn get_site_addr() -> String {
    // let conf = get_configuration(None).await.unwrap();
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    format!("http://{}:{}/api", addr.ip(), addr.port())
}




pub async fn fetch_regions() -> Vec<SelectOpt> {
    //gloo_timers::future::TimeoutFuture::new(1000).await;
    match fetch_api::<Vec<Region>>(&format!("{}/region", get_site_addr())).await {
        Some(regions) => regions
            .into_iter()
            .map(|r| SelectOpt::new(r.code, r.name))
            .collect::<Vec<_>>(),
        None => Vec::new(),
    }
}

// #[cfg(not(feature = "ssr"))]
// pub fn fetch_api<T>(path: &str) -> impl std::future::Future<Output = Option<T>> + Send + '_
// where
//     T: Serialize + DeserializeOwned,
// {
//     use leptos::prelude::on_cleanup;
//     use send_wrapper::SendWrapper;
// 
//     SendWrapper::new(async move {
//         let abort_controller = SendWrapper::new(web_sys::AbortController::new().ok());
//         let abort_signal = abort_controller.as_ref().map(|a| a.signal());
// 
//         // abort in-flight requests if, e.g., we've navigated away from this page
//         on_cleanup(move || {
//             if let Some(abort_controller) = abort_controller.take() {
//                 abort_controller.abort()
//             }
//         });
// 
//         gloo_net::http::Request::get(path)
//             .abort_signal(abort_signal.as_ref())
//             .send()
//             .await
//             .map_err(|e| log::error!("{e}"))
//             .ok()?
//             .json()
//             .await
//             .ok()
//     })
// }

// #[cfg(feature = "ssr")]
// async fn fetch_api<T>(path: &str) -> Option<T>
// where
//     T: Serialize + DeserializeOwned,
// {
//     reqwest::get(path)
//         .await
//         .map_err(|e| log!("API error: {e}"))
//         .ok()?
//         .json()
//         .await
//         .ok()
// }

pub fn fetch_api<T>(
    path: &str,
) -> impl std::future::Future<Output = Option<T>> + Send + '_
where
    T: Serialize + DeserializeOwned,
{
    SendWrapper::new(async move {
        let abort_controller =
            SendWrapper::new(web_sys::AbortController::new().ok());
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());

        // abort in-flight requests if, e.g., we've navigated away from this page
        on_cleanup(move || {
            if let Some(abort_controller) = abort_controller.take() {
                abort_controller.abort()
            }
        });

        gloo_net::http::Request::get(path)
            .header("Access-Control-Allow-Origin", "*")
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| logging::error!("{e}"))
            .ok()?
            .json()
            .await
            .ok()
    })
}