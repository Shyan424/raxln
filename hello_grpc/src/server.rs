use std::pin::Pin;

use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Response, Status, Request, Streaming};

use crate::hello::{HelloResponse, HelloRequest};
use crate::hello::hello_to_who_server::{HelloToWho, HelloToWhoServer};

#[derive(Debug)]
struct HelloService;

#[tonic::async_trait]
impl HelloToWho for HelloService {
    async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let hello_req = request.get_ref();
        let hello_res_str = format!("Hello {}", hello_req.name);
        
        let hello_res = HelloResponse {str: hello_res_str};

        Ok(Response::new(hello_res))
    }

    type helloServerStreamStream = ReceiverStream<Result<HelloResponse, Status>>;

    // server端stream
    async fn hello_server_stream(&self, request: Request<HelloRequest>) -> Result<Response<Self::helloServerStreamStream>, Status> {
        let hello_req = request.get_ref();
        let hello_res_str = format!("Hello Server Stream {}", hello_req.name);

        let (tx, rx) = mpsc::channel(5);

        tokio::spawn(async move {
            for str in hello_res_str.chars() {
                tx.send(Ok(HelloResponse { str: str.to_string() })).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn hello_client_stream(&self, request: Request<Streaming<HelloRequest>>) -> Result<Response<HelloResponse>, Status> {
        let mut stream = request.into_inner();
        let mut res_str = String::from("Hello Client Stream");

        // next need StreamExt
        while let Some(Ok(req)) = stream.next().await {
            res_str.push_str(&req.name);
        };
    
        Ok(Response::new(HelloResponse{ str: res_str }))
    }

    type helloAllStreamStream = Pin<Box<dyn Stream<Item = Result<HelloResponse, Status>> + Send  + 'static>>;

    async fn hello_all_stream(&self, request: Request<Streaming<HelloRequest>>) -> Result<Response<Self::helloAllStreamStream>, Status> {
        let mut stream = request.into_inner();
        let mut res_str = String::from("Hello All Stream ");

        let out = async_stream::try_stream! {
            while let Some(Ok(req)) = stream.next().await {
                res_str.push_str(&req.name);

                yield HelloResponse { str: res_str.clone() };
            }
        };

        Ok(Response::new(Box::pin(out) as Self::helloAllStreamStream))
    }
}

pub async fn start_hello_grpc(ip: String) -> Result<(), Box<dyn std::error::Error>> {
    let ser = HelloToWhoServer::new(HelloService);
    Server::builder()
        .add_service(ser)
        .serve(ip.parse().unwrap()).await?;

    Ok(())
}