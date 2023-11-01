use tower_http::trace::OnRequest;
use tracing::info;


#[derive(Clone, Debug)]
pub struct TraceRequest {}

impl TraceRequest {
    pub fn new() -> Self {
        TraceRequest {}
    }
}

impl<B> OnRequest<B> for TraceRequest {
    fn on_request(&mut self, request: &axum::http::Request<B>, _: &tracing::Span) {
        let method = request.method();
        let uri = request.uri();

        info!("method {} uri {} ", method, uri);
    }
}