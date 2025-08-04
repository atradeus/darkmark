use axum_macros::FromRef;
use leptos::prelude::*;
use sqlx_postgres::PgPool;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}

