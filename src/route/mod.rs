use self::hello::hello_grpc_connect;

pub mod test;
pub mod hello;
pub mod res_error;

pub async fn grpc_connect() {
    hello_grpc_connect().await;
}