use axum::extract::Request;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;

use tracing::info;

use crate::jwt::eddsa::EdDsaJwt;


use super::error::AuthError;

// https://github.com/wpcodevo/rust-axum-jwt-auth/blob/master/src/jwt_auth.rs
pub async fn jwt_authorization(request: Request, next: Next) -> Result<Response, AuthError> {
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
        AuthError::WithoutToken
    })?;

    let claims = EdDsaJwt::default().validate(&token)
        .map_err(|e| e)?;

    info!("hi {}", claims.id);

    let response = next.run(request).await;

    // 跑完邏輯ㄉ時候做

    Ok(response)
}
