use axum::Json;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};

use super::CONNECT;

#[derive(Serialize)]
pub struct HelloResponse {
    messgae: Vec<String>
}

#[derive(Deserialize)]
pub struct HelloRequest {
    names: Vec<String>
}

pub async fn hello(Json(req): Json<HelloRequest>) -> (StatusCode, Json<HelloResponse>) {
    let res = CONNECT.get().unwrap()
        .hello(req.names[0].clone()).await;

    match res {
        Ok(s) => (StatusCode::OK, Json(HelloResponse{ messgae: vec![s] })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(HelloResponse{ messgae: vec![String::from("err")] }))
    }
}

pub async fn hello_server_stream(Json(req): Json<HelloRequest>) -> (StatusCode, Json<HelloResponse>) {
    let res = CONNECT.get().unwrap()
        .hello_server_stream(req.names[0].clone()).await;

    match res {
        Ok(v) => (StatusCode::OK, Json(HelloResponse{ messgae: v })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(HelloResponse{ messgae: vec![String::from("err")] }))
    }
}

pub async fn hello_client_stream(Json(req): Json<HelloRequest>) -> (StatusCode, Json<HelloResponse>) {
    let res = CONNECT.get().unwrap()
        .hello_client_stream(req.names).await;

    match res {
        Ok(s) => (StatusCode::OK, Json(HelloResponse{ messgae: vec![s] })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(HelloResponse{ messgae: vec![String::from("err")] }))
    }
}

pub async fn hello_all_stream(Json(req): Json<HelloRequest>) -> (StatusCode, Json<HelloResponse>) {
    let res = CONNECT.get().unwrap()
        .hello_all_stream(req.names).await;

    match res {
        Ok(v) => (StatusCode::OK, Json(HelloResponse{ messgae: v })),
        Err(_) => (StatusCode::BAD_REQUEST, Json(HelloResponse{ messgae: vec![String::from("err")] }))
    }
}