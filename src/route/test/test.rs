use std::collections::HashMap;

use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, Path};
use axum::http::header::AUTHORIZATION;
use axum::http::{StatusCode, HeaderMap};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::jwt::eddsa::EdDsaJwt;


pub async fn get_test() -> (StatusCode, String) {
    (StatusCode::OK, String::from("Get OMG"))
}

pub async fn post_test() -> (StatusCode, String) {
    (StatusCode::OK, String::from("POST OMG"))
}

pub async fn query_test(Query(query): Query<HashMap<String, String>>) -> (StatusCode, String) {
    let query_t = query.get("t").unwrap();

    (StatusCode::OK, format!("pass query {query_t}"))
}

// 使用 struct 轉換時 沒有用 Option 的參數會是必要的 沒有該參數會報錯
#[derive(Deserialize)]
pub struct QueryStruct {
    q: String
}

pub async fn query_struct_test(Query(query): Query<QueryStruct>) -> (StatusCode, String) {
    (StatusCode::OK, format!("query struct {}", query.q))
}

pub async fn path_test(Path(path): Path<String>) -> (StatusCode, String) {
    (StatusCode::OK, format!("pass path {path}"))
}

#[derive(Debug, Deserialize)]
pub struct ReqData {
    name: String,
    age: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResData {
    msg: String
}

pub async fn json_struct_test(req_data: Result<Json<ReqData>, JsonRejection>) -> (StatusCode, Json<ResData>) {
    match req_data {
        Ok(rq_data) => {
            if let Some(age) = rq_data.age {
                (StatusCode::OK, Json(ResData{msg: format!("name is {} age {}", rq_data.name, age)}))
            } else {
                (StatusCode::OK, Json(ResData{msg: format!("name is {}", rq_data.name)}))
            }
        },
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(ResData{msg: format!("Error: {e}")}))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Login {
    id: Option<String>
}

#[instrument]
pub async fn login(Query(query): Query<Login>) -> (HeaderMap, Json<ResData>) {
    let id = query.id.map_or(String::from("test_id"), |i| i);
    info!("id {}", id);
    let token = EdDsaJwt::default().signature(&id).unwrap();
    let mut header = HeaderMap::new();
    header.insert(AUTHORIZATION, format!("Bearer {token}").parse().unwrap());

    let res_data = ResData{msg: String::from("ok")};

    (header, Json(res_data))
}