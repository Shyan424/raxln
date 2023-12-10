use tonic::Code::InvalidArgument;
use tonic::{Request, Status};
use tonic::transport::Channel;

use crate::error::Error;
use crate::hello::HelloRequest;
use crate::hello::hello_to_who_client::HelloToWhoClient;

#[derive(Debug)]
pub struct HelloClient {
    client: HelloToWhoClient<Channel>
}

impl HelloClient {
    pub async fn connect(addr: String) -> Result<Self, Error> {
        let addr = addr.clone();
        let channel = Channel::from_shared(addr)
            .map_err(|_| Error::UriError)?
            .connect().await;

        match channel {
            Ok(c) => Ok(HelloClient { client: HelloToWhoClient::new(c) }),
            Err(_) => Err(Error::ConnectFail)
        }
    }

    pub async fn hello(&self, name: String) -> Result<String, Error> {
        let hello_res = self.client.clone()
            .hello(HelloRequest { name })
            .await;
        match hello_res {
            Ok(r) => Ok(r.into_inner().str),
            Err(e) => Err(handle_tonic_status(e))?
        }
    }

    pub async fn hello_server_stream(&self, name: String) -> Result<Vec<String>, Error> {
        let mut res_str = Vec::new();
        let res = self.client.clone()
            .hello_server_stream(HelloRequest{ name })
            .await;
        let mut stream = res.map_err(handle_tonic_status)?.into_inner();
        while let Ok(Some(res)) = stream.message().await {
            res_str.push(res.str);
        };

        Ok(res_str)
    }

    pub async fn hello_client_stream(&self, names: Vec<String>) -> Result<String, Error> {
        let mut hello_req = Vec::new();
        for name in names {
            hello_req.push(HelloRequest{ name });
        }
    
        let req = Request::new(tokio_stream::iter(hello_req));    
        match self.client.clone().hello_client_stream(req).await {
            Ok(res) => Ok(res.into_inner().str),
            Err(e) => Err(handle_tonic_status(e))?
        }
    }

    pub async fn hello_all_stream(&self, names: Vec<String>) -> Result<Vec<String>, Error> {
        let out = async_stream::stream! {
            for name in names {
                yield HelloRequest{ name }
            }
        };

        let res = self.client.clone()
            .hello_all_stream(Request::new(out)).await;
        let mut stream = res.map_err(handle_tonic_status)?.into_inner();
        let mut res_str = Vec::new();
        while let Ok(Some(hello_res)) = stream.message().await {
            res_str.push(hello_res.str);
        };

        Ok(res_str)
    }
}

fn handle_tonic_status(status: Status) -> Error {
    if status.code() == InvalidArgument {
        Error::RequestDataError(String::from(status.message()))
    } else {
        Error::IDontKnow(String::from("hehe"))
    }
}