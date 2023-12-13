mod router;
pub mod hello_grpc;

pub use router::hello_grpc_router;

use std::sync::OnceLock;

use ::hello_grpc::client::HelloClient;

static CONNECT: OnceLock<HelloClient> = OnceLock::new();

pub async fn hello_grpc_connect() {
    CONNECT.set(
        // https://github.com/hyperium/tonic/issues/279
        HelloClient::connect(String::from("https://[::1]:50061"))
            .await.expect("grpc client error")
    ).expect("connect hello grpc client error");
}