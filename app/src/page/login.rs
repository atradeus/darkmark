use crate::api;
use crate::api::Error;
use api::auth;
use auth::authorize;
use leptos::ev;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use lib::user::Credentials;

#[component]
pub fn Login() -> impl IntoView {
    let (login_error, set_login_error) = signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = signal(false);

    let (password, set_password) = signal(String::new());
    let (email, set_email) = signal(String::new());

    let disabled = Signal::derive(move || wait_for_response.get());
    
    let login_action = Action::new(move |(e, p): &(String, String)| {
        let email = e.clone();
        let password = p.clone();
        async move {
            let credentials = Credentials {
                email,
                password,
                remember: None,
            };
            
            set_wait_for_response.update(|w| *w = true);
            let login_result = authorize(&credentials).await;
            set_wait_for_response.update(|w| *w = false);

            match login_result {
                Ok(()) => {
                    log!("login successfully");
                    set_login_error.update(|e| *e = None);
                    let navigate = use_navigate();
                    navigate("/", Default::default());
                }
                Err(err) => {
                    let msg = match err {
                        Error::Fetch(e) => {
                            format!("{e:?}")
                        }
                        Error::LoginError(e) => {
                            format!("{e:?}")
                        }
                        _ => {
                            format!("Unknown Error: {}", err)
                        }
                    };
                    log::error!("Unable to login with {}: {msg}", credentials.email);

                    let msg = "Invalid username or password".to_string();

                    set_login_error.update(|e| *e = Some(msg));
                }
            }
        }
    });

    let button_is_disabled = Signal::derive(move || {
        disabled.get() || password.get().is_empty() || email.get().is_empty()
    });

    let app_name = t!("app_name").to_string();

    view! {
        // <div class="flex h-screen w-full items-center justify-center bg-gray-900 bg-cover bg-no-repeat" style="background-image:url('/img/gym_bg.jpg')">
        <div class="flex h-screen w-full items-center justify-center bg-gray-900 bg-cover bg-no-repeat">
            <div class="rounded-xl bg-gray-800 bg-opacity-50 px-16 py-10 shadow-lg backdrop-blur-md max-sm:px-8">
                <div class="text-white">
                    <div class="mb-8 flex flex-col items-center">
                        <h1 class="mb-2 text-2xl">{app_name}</h1>
                        <span class="text-gray-300">Login</span>
                    </div>
                    <form on:submit=|ev| {
                        ev.prevent_default()
                    }>
                        {move || {
                            login_error
                                .get()
                                .map(|err| {
                                    view! { <p class="text-center text-red-600 pb-2">{err}</p> }
                                })
                        }} <div class="mb-4 text-lg">
                            <input
                                class="rounded-2xl border-none bg-blue-400 bg-opacity-50 px-6 py-2 text-center text-inherit placeholder-slate-200 shadow-lg outline-none backdrop-blur-md"
                                type="email"
                                required
                                placeholder="Email"
                                prop:disabled=move || disabled.get()
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    let val = event_target_value(&ev);
                                    set_email.update(|v| *v = val);
                                }
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_email.update(|v| *v = val);
                                }
                            />
                        </div> <div class="mb-4 text-lg">
                            <input
                                class="rounded-2xl border-none bg-blue-400 bg-opacity-50 px-6 py-2 text-center text-inherit placeholder-slate-200 shadow-lg outline-none backdrop-blur-md"
                                type="password"
                                required
                                placeholder="Password"
                                prop:disabled=move || disabled.get()
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    match &*ev.key() {
                                        "Enter" => {
                                            login_action
                                                .dispatch((email.clone().get(), password.clone().get()));
                                        }
                                        _ => {
                                            let val = event_target_value(&ev);
                                            set_password.update(|p| *p = val);
                                        }
                                    }
                                }
                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_password.update(|p| *p = val);
                                }
                            />
                        </div> <div class="mt-8 flex justify-center text-lg text-black">
                            <button
                                class="rounded-2xl bg-yellow-400 bg-opacity-50 px-10 py-2 text-white shadow-xl backdrop-blur-md transition-colors duration-300 hover:bg-yellow-600"
                                prop:disabled=move || button_is_disabled.get()
                                // on:click=move |_| {let _ = dispatch_action.clone()();}
                                on:click=move |_| {
                                    login_action
                                        .dispatch((email.clone().get(), password.clone().get()));
                                }
                            >
                                Login
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
