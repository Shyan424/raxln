pub mod server;
pub mod client;
pub mod error;


mod hello {
    tonic::include_proto!("hello");
}