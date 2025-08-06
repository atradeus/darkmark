use leptos::logging;
use leptos::prelude::get_configuration;
use reactive_graph::owner::on_cleanup;
use send_wrapper::SendWrapper;
use thiserror::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod auth;

// type Result<T> = std::result::Result<T, Error>;

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

pub fn get_site_addr() -> String {
    // let conf = get_configuration(None).await.unwrap();
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    format!("http://{}:{}/api", addr.ip(), addr.port())
}

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