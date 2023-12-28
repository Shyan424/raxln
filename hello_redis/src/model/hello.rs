use redis_macros::{FromRedisValue, ToRedisArgs};


#[derive(Debug, serde::Serialize, serde::Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Hello {
    pub name: String,
}

// impl FromRedisValue for Hello {
//     fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
//         let s = from_redis_value::<String>(v)?;
//         serde_json::from_str::<Hello>(s.as_str()).map_err(|_| RedisError::from((ErrorKind::TypeError, "")))
//     }
// }
