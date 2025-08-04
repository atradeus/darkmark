use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Error};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum::extract::State;
use axum::Json;
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, query_as};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx_postgres::{PgPool, Postgres};

use lib::user::{Credentials, Role, User};

use crate::error::AppError;
use crate::error::AppError::{ApiError, LoginError};

// #[axum_macros::debug_handler]
pub async fn auth(
    mut auth: AuthSession<AppUser, String, SessionPgPool, PgPool>,
    State(pool): State<PgPool>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<User>, AppError> {
    log::debug!("3 auth {credentials}, AppUser: {:?}, authenticated: {}", auth.current_user, auth.is_authenticated());

    match check_credentials(credentials.clone(), pool).await {
        Err(e) => Err(e),
        Ok(app_user) => {
            auth.login_user(credentials.email);
            auth.remember_user(credentials.remember.or(Some(false)).unwrap());
            auth.current_user = Some(app_user.clone());

            Ok(Json(app_user.into()))
        }
    }
}

async fn check_credentials(credentials: Credentials, pool: PgPool) -> Result<AppUser, AppError> {
    let app_user = match AppUser::get_by_email(credentials.email, &pool).await {
        Ok(a) => a,
        Err(e) => return Err(LoginError(e.to_string()))
    };

    log::debug!("check credentials for user: {:?}", app_user);

    let password = credentials.password.as_bytes();
    let password_hash = PasswordHash::new(&app_user.password).unwrap();

    match Argon2::default().verify_password(password, &password_hash) {
        Ok(_) => {
            log::debug!("Password match");
            Ok(app_user)
        }
        Err(e) => {
            log::debug!("Password error: {e}");
            Err(LoginError("Password does not match.".to_string()))
        }
    }
}

pub async fn get_users(users: Vec<i32>, pool: PgPool) -> Result<Vec<User>, AppError> {
    let result: Vec<AppUser> =
        match query_as("select * from users where user_id in ($1)")
            .bind(users)
            .fetch_all(&pool)
            .await {
            Ok(r) => r,
            Err(e) => return Err(ApiError(format!("Error getting users error={}", e)))
        };

    Ok(result.iter()
        .map(|r| r.clone().into())
        .collect())
}

#[async_trait]
impl Authentication<AppUser, String, Pool<Postgres>> for AppUser {
    async fn load_user(email: String, pool: Option<&Pool<Postgres>>) -> Result<AppUser, Error> {
        log::debug!("load_user {email}");
        match AppUser::get_by_email(email.clone(), pool.unwrap()).await {
            Ok(a) => Ok(a),
            Err(_) => Err(anyhow::anyhow!("Error retrieving user {}", email))
        }
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

#[derive(Debug, FromRow, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppUser {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    #[sqlx(skip)]
    pub anonymous: bool,
    #[sqlx(skip)]
    pub roles: HashSet<Role>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<User> for AppUser {
    fn into(self) -> User {
        User {
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            anonymous: self.anonymous,
            roles: self.roles,
        }
    }
}

impl AppUser {
    pub async fn get_by_user_id(id: i32, pool: &Pool<Postgres>) -> Result<Self, Error> {
        let app_user: AppUser = match query_as("select * from users where id = $1")
            .bind(id.clone())
            .fetch_one(pool)
            .await {
            Ok(r) => r,
            Err(e) => return Err(anyhow!(format!("Error getting user id={id}, error={e}")))
        };
        AppUser::get_user(app_user, pool).await
    }

    pub async fn get_by_email(email: String, pool: &Pool<Postgres>) -> Result<Self, Error> {
        let app_user: AppUser = match query_as("select * from users where email = $1")
            .bind(email.clone())
            .fetch_one(pool)
            .await {
            Ok(r) => r,
            Err(e) => return Err(anyhow!(format!("Error getting user email={email}, error={e}")))
        };
        AppUser::get_user(app_user, pool).await
    }

    async fn get_user(mut app_user: AppUser, pool: &Pool<Postgres>) -> Result<Self, Error> {
        let user_id = app_user.clone().id;

        let sql_user_perms: Option<Vec<SqlPermissionRole>> = match query_as(
//language=SQL
r#"
        select role
        FROM users a,
             user_role b,
             role c
        WHERE a.id = b.user_id
          and b.role_id = c.id
          and b.user_id = a.id
          and a.id = $1
   "#
        )
            .bind(user_id.clone())
            .fetch_all(pool)
            .await {
            Ok(r) => Some(r),
            Err(e) => return Err(anyhow!("Error getting user permissions id={user_id}, error={e}"))
        };

        app_user.roles = map_perms(sql_user_perms);
        app_user.anonymous = app_user.email.is_empty();

        Ok(app_user)
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionRole {
    pub role: String,
}

fn map_perms(sql_user_perms: Option<Vec<SqlPermissionRole>>) -> HashSet::<Role> {
    if let Some(user_perms) = sql_user_perms {
        user_perms
            .into_iter()
            .map(|x| Role::from_str(&x.role).unwrap())
            .collect::<HashSet<Role>>()
    } else {
        HashSet::<Role>::new()
    }
}