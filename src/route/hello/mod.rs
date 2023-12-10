mod router;
pub mod hello_grpc;

pub use router::hello_grpc_router;

use std::sync::OnceLock;

use ::hello_grpc::client::HelloClient;

static CONNECT: OnceLock<HelloClient> = OnceLock::new();

pub async fn hello_grpc_connect() {
    CONNECT.set(
        HelloClient::connect(String::from("http://[::1]:50061")).await.expect("grpc client error")
    ).expect("connect hello grpc client error");
}