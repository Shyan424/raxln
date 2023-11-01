
use axum::{Router, middleware};
use axum::routing::{get, post};

use crate::jwt;

use super::test::{get_test, post_test, query_test, query_struct_test, path_test, json_struct_test, login};

pub fn test_router() -> Router {
    Router::new()
        .route("/", get(get_test).post(post_test))
        .route("/query", get(query_test))
        .route("/squery", get(query_struct_test))
        .route("/om", post(json_struct_test))
        .route("/:path", get(path_test))
        .layer(middleware::from_fn(jwt::middleware::jwt_authorization))
        // layer 只會影響上面的，不會影響下面的
        .route("/login", get(login))
}