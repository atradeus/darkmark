use leptos::logging::log;
use lib::user::Credentials;
use crate::api::{Error, Error::LoginError, get_site_addr};
use leptos::prelude::on_cleanup;
use send_wrapper::SendWrapper;

pub async fn authorize(credentials: &Credentials) -> Result<(), Error> {
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

        let url = format!("{}/auth", get_site_addr());
    
        log!("auth url: {url}");
        
        let result = gloo_net::http::Request::post(&url)
            .header("Access-Control-Allow-Origin", "*")
            .abort_signal(abort_signal.as_ref())
            .json(&credentials)
            .unwrap()
            .send()
            .await;
        
        let response = match result {
            Ok(r) => r,
            Err(e) => return Err(Error::Fetch(e))
        };
        
        if response.ok() {
            Ok(())
        } else {
            let t = response.text().await.unwrap_or("Unknown Error".to_string());
            log::debug!("Login error {}", t.clone());
            Err(LoginError(t))
        }
    }).await
}
