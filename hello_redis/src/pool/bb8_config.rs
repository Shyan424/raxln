use async_trait::async_trait;
use redis::aio::Connection;
use redis::cluster_async::ClusterConnection;

use crate::Error;

use super::{RedisClusterClient, RedisClient};

#[async_trait]
impl bb8::ManageConnection for RedisClient {
    type Connection = Connection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match self.client.get_async_connection().await {
            Ok(c) => Ok(c),
            Err(_) => Err(Error::ConnectError(String::from("pool connect error")))
        }
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let ping: String = redis::cmd("PING")
            .query_async(conn)
            .await
            .map_err(|_| Error::QueryError(String::from("Query Error")))?;
        
        match ping.as_str() {
            "PONG" => Ok(()),
            _ => Err(Error::QueryError(String::from("Data Error")))
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisClusterClient {
    type Connection = ClusterConnection;
    type Error = Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match self.client.get_async_connection().await {
            Ok(c) => Ok(c),
            Err(_) => Err(Error::ConnectError(String::from("pool connect error")))
        }
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let ping: String = redis::cmd("PING")
            .query_async(conn)
            .await
            .map_err(|_| Error::QueryError(String::from("Query Error")))?;
        
        match ping.as_str() {
            "PONG" => Ok(()),
            _ => Err(Error::QueryError(String::from("Data Error")))
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}