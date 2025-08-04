use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    LoginError(String),
    NotAuthenticated,
    DatabaseError(String),
    ApiError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = match self {
            AppError::LoginError(s) => s,
            AppError::NotAuthenticated => "Not Authenticated".to_string(),
            AppError::DatabaseError(s) => s,
            AppError::ApiError(s) => s,
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}