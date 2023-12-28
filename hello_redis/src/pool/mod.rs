use bb8::{Pool, ManageConnection, PooledConnection};
use redis::{Client, Cmd, FromRedisValue};
use redis::cluster::ClusterClient;

use crate::Error;


mod bb8_config;


pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(url: String) -> Result<Self, Error> {
        let client = redis::Client::open(url)
            .map_err(|e| Error::ClientBuildError(e.to_string()))?;

        Ok(RedisClient { client })
    }
}

pub struct RedisClusterClient {
    client: ClusterClient,
}

impl RedisClusterClient {
    pub fn new(url: Vec<String>) -> Result<Self, Error> {
        let client = redis::cluster::ClusterClient::builder(url)
            .build()
            .map_err(|e| Error::ClientBuildError(e.to_string()))?;

        Ok(RedisClusterClient { client })
    }
}

pub struct RedisPoolConfig {
    pub max_size: u32,
    pub min_idle: Option<u32>
}

impl RedisPoolConfig {
    pub fn new() -> Self {
        RedisPoolConfig {
            max_size: 10,
            min_idle: None,
        }
    }
}

pub async fn redis_pool<M: ManageConnection>(client: M, config: RedisPoolConfig) -> Result<RedisPool<M>, Error> {
    let p = bb8::Pool::builder()
        .max_size(config.max_size)
        .min_idle(config.min_idle)
        .build(client)
        .await
        .map_err(|_| Error::ClientBuildError(String::from("Build Connection Pool Error")))?;

    Ok(RedisPool { pool: p })
}

pub struct RedisPool<M: ManageConnection> {
    pool: Pool<M>
}

impl <M: ManageConnection> RedisPool<M> {
    pub async fn get_conn(&self) -> PooledConnection<'_, M> {
        self.pool.get().await.unwrap()
    }

    pub async fn query<T>(&self, cmd: Cmd) -> Result<T, Error>
    where
        T: FromRedisValue,
        M::Connection: redis::aio::ConnectionLike,
    {
        cmd.query_async(&mut *self.get_conn().await)
            .await
            .map_err(|e| Error::QueryError(e.to_string()))
    }
}

