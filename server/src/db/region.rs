use crate::error::AppError;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_macros::debug_handler;
use chrono::{DateTime, Utc};
use leptos::prelude::ServerFnError;
use lib::Region;
use postgres::error::DbError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_postgres::PgPool;

pub async fn list_regions(State(pool): State<PgPool>) -> Result<Json<Vec<Region>>, AppError> {
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
