use axum::Json;
use axum::http::{Request, StatusCode, header};
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tracing::info;

use crate::jwt::eddsa::EdDsaJwt;


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    msg: String
}

// https://github.com/wpcodevo/rust-axum-jwt-auth/blob/master/src/jwt_auth.rs
pub async fn jwt_authorization<B>(request: Request<B>, next: Next<B>) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    // in ㄉ時候做
    let token = request.headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });

    let token = token.ok_or_else(|| {
        let error_msg = ErrorResponse {
            msg: String::from("Without token")
        };

        (StatusCode::UNAUTHORIZED, Json(error_msg))
    })?;

    let claims = EdDsaJwt::default().validate(&token)
        .map_err(|_| {
            let err = ErrorResponse {
                msg: String::from("validate fail")
            };

            (StatusCode::UNAUTHORIZED, Json(err))
        })?;

    info!("hi {}", claims.id);

    let response = next.run(request).await;

    // 跑完邏輯ㄉ時候做

    Ok(response)
}
