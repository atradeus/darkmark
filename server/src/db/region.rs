use crate::error::AppError;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_macros::debug_handler;
use axum_session_auth::AuthSession;
use axum_session_sqlx::SessionPgPool;
use chrono::{DateTime, Utc};
use leptos::prelude::ServerFnError;
use lib::user::User;
use lib::Region;
use log::debug;
use postgres::error::DbError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool};
use sqlx_postgres::{PgPool, Postgres};
use crate::user::AppUser;

// pub async fn list_regions(State(pool): State<PgPool>) -> Result<Json<Vec<Region>>, AppError> {
pub async fn list_regions(
    auth: AuthSession<AppUser, String, SessionPgPool, PgPool>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Region>>, AppError> {
    if !auth.is_authenticated() {
        return Err(AppError::LoginError("Not logged in".to_string()))
    }

    #[derive(Debug, FromRow, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
    struct DbRegion {
        pub id: i32,
        pub code: String,
        pub name: String,
        pub geography: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    let recs = match sqlx::query_as::<_, DbRegion>(
        r#"
SELECT *
FROM region
ORDER BY code
        "#,
    )
    .fetch_all(&pool)
    .await
    {
        Ok(recs) => recs,
        Err(e) => {
            return Err(AppError::DatabaseError(
                "Error fetching regions".to_owned() + &e.to_string(),
            ))
        }
    };

    let mut regions = Vec::with_capacity(recs.len());
    for r in recs {
        regions.push(Region {
            code: r.code,
            name: r.name,
            geography: r.geography,
        });
    }

    Ok(Json(regions))
}
