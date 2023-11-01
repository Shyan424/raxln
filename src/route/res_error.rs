use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::jwt::error::AuthError;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    msg: String
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, msg) = match self {
            AuthError::CreateError => (StatusCode::INTERNAL_SERVER_ERROR, "create token error"),
            AuthError::ValidateError => (StatusCode::UNAUTHORIZED, "validate error"),
            AuthError::WithoutToken => (StatusCode::BAD_REQUEST, "without token")
        };

        let err_res = ErrorResponse {
            msg: String::from(msg)
        };

        (status_code, Json(err_res)).into_response()
    }
}