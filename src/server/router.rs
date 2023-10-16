use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::http::header;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use tower_http::cors::{CorsLayer, Any};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;


// https://github.com/tower-rs/tower-http/blob/master/examples/axum-key-value-store/src/main.rs
pub fn router() -> Router {

    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let middleware = ServiceBuilder::new()
        // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
        .sensitive_request_headers(sensitive_headers.clone())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .compression()
        .layer(CorsLayer::new().allow_origin(Any))
        // Box the response body so it implements `Default` which is required by axum
        // 要加在 TraceLayer 的上面
        .map_response_body(axum::body::boxed)
        .layer(
            TraceLayer::new_for_http()
                // .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                //     tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                // })
                // .make_span_with(DefaultMakeSpan::new().include_headers(true).level(Level::INFO))
                // .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros).level(Level::INFO))
        );

    Router::new()
        // .route("/", get(hello_world))
        // layer 只會影響上面的，不會影響下面的
        .layer(middleware)
}
