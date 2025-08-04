#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod cloud;
pub mod db;
pub mod error;
pub mod state;
pub mod user;

use axum::extract::{Request, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::response::{Html, IntoResponse};
// use axum_macros::debug_handler;
use crate::cloud::aws;
use crate::state::AppState;
use crate::user::{auth, AppUser};
use app::*;
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer};
use axum_session_sqlx::SessionPgPool;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{
    file_and_error_handler, generate_route_list, handle_server_fns_with_context, LeptosRoutes,
};
use sqlx::Pool;
use sqlx_postgres::{PgPool, PgPoolOptions, Postgres};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
// #[axum::debug_handler]
async fn main() {
    /*
    env_logger::init();

    //TODO fix database env
    let db_url = "postgres://darkmark:password@athena.local:5432/darkmark";
    let pool = match PgPoolOptions::new()
            .max_connections(5)
            //.connect(env!("DATABASE_URL")).await {
            .connect(db_url).await {
            Ok(p) => p,
            Err(_) => panic!("Server dead: Failed to connect to database")
        };

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
    };

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/api/{*fn_name}", post(server_func_handler))
        .route("/region", get(db::region::list_regions))
        .layer(cors)
        .leptos_routes(&app_state, routes, {
            let leptos_options = app_state.clone().leptos_options;
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    */

    env_logger::init();

    let db_url = "postgres://darkmark:darkmark@athena:5432/darkmark";
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        // .connect(&std::env::var("DATABASE_URL").unwrap())
        .connect(db_url)
        .await
    {
        Ok(p) => p,
        Err(_) => panic!("Failed to connect to database"),
    };

    // For now, manually migrating changes
    // if let Err(e) = migrate!("src/db/migrations").run(&pool).await {
    //     eprintln!("{e:?}");
    // }

    //This Defaults as normal Cookies.
    //To enable Private cookies for integrity, and authenticity please check the next Example.
    let session_config = SessionConfig::default().with_table_name("user_session");
    let auth_config = AuthConfig::<String>::default().with_anonymous_user_id(None);

    // create SessionStore and initiate the database tables
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config)
            .await
            .unwrap();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    // build our application with a route
    // let app = Router::new()
    //     .route("/ping", get(|| async { "pong" }))
    //     .route("/api/auth", post(user::auth))
    //     .layer(cors)
    //     // .route(
    //     //     "/api/*fn_name",
    //     //     get(server_fn_handler).post(server_fn_handler),
    //     // )
    //     .layer(
    //         AuthSessionLayer::<AppUser, String, SessionPgPool, Pool<Postgres>>::new(Some(pool.clone()))
    //             .with_config(auth_config),
    //     )
    //     .leptos_routes(&app_state, routes, App)
    //     .fallback(file_and_error_handler)
    //     .with_state(app_state)
    //     .layer(SessionLayer::new(session_store))
    //     ;

    let app = Router::new()
        .route("/ping", get(|| async { "pong" }))
        .route("/api/auth", post(auth))
        .route("/api/region", get(db::region::list_regions))
        .route("/api/{*fn_name}", post(server_func_handler))
        .layer(cors)
        .layer(
            AuthSessionLayer::<AppUser, String, SessionPgPool, Pool<Postgres>>::new(Some(
                pool.clone(),
            ))
            .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store.clone()))
        .leptos_routes(&app_state, routes, {
            let leptos_options = app_state.clone().leptos_options;
            move || shell(leptos_options.clone())
        })
        .fallback(file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// #[cfg(feature="ssr")]
async fn server_func_handler(
    auth_session: AuthSession<AppUser, String, SessionPgPool, PgPool>,
    //session: tower_sessions::Session,
    State(app_state): State<AppState>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            // AuthSession has a session within it, but you can still use the session extractor
            // directly to get access to the same session. This holds the `user` field, which will be
            // `Some(<userdata>)` if somebody is logged in, or `None` otherwise.
            provide_context(auth_session.clone());

            // This isn't strictly necessary, but if you want to use tower-sessions without axum_login,
            // this is how you would pass the session objects into your server functions.
            //provide_context(session.clone());

            // This holds the connection pool and leptos options
            provide_context(app_state.clone());

            // This is the data from the `server_config.toml` file
            //provide_context(app_state.server_config.clone());
        },
        req,
    )
    .await
}
