
#[tokio::main]
async fn main() {
    hello_grpc::server::start_hello_grpc(String::from("[::1]:50061")).await.expect("start hello grpc server error");
}