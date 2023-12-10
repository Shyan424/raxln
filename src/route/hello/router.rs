use axum::Router;
use axum::routing::get;

use super::hello_grpc::{hello, hello_server_stream, hello_client_stream, hello_all_stream};


pub fn hello_grpc_router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/sstream", get(hello_server_stream))
        .route("/cstream", get(hello_client_stream))
        .route("/astream", get(hello_all_stream))
}