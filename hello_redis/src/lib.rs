pub mod pool;
pub mod error;
pub mod model;

pub use error::Error;
use pool::{RedisPool, RedisClusterClient, redis_pool, RedisPoolConfig};

pub async fn get_cluster_pool(url: Vec<String>, config: RedisPoolConfig) -> Result<RedisPool<RedisClusterClient>, Error> {
    redis_pool(RedisClusterClient::new(url)?, config).await
}

#[cfg(test)]
mod test {
    use redis::Cmd;
    use tokio::sync::OnceCell;

    use crate::get_cluster_pool;
    use crate::model::hello::Hello;
    use crate::pool::{RedisPoolConfig, RedisPool, RedisClusterClient};

    async fn get_pool() -> &'static RedisPool<RedisClusterClient> {
        static REDIS: OnceCell<RedisPool<RedisClusterClient>> = OnceCell::const_new();
        
        REDIS.get_or_init(|| async {
            let urls = vec![String::from("redis://:rpassword@127.0.0.1:7000/")];
            let config = RedisPoolConfig::new();
            get_cluster_pool(urls, config).await.unwrap()
        }).await
    }

    #[tokio::test]
    #[ignore]
    async fn conn_test() {
        let pool = get_pool().await;

        let pong = redis::cmd("PING")
            .arg(1)
            .query_async::<_, String>(&mut *pool.get_conn().await)
            .await
            .unwrap();

        assert_eq!("1", pong.as_str());
    }

    #[tokio::test]
    #[ignore]
    async fn struct_test() {
        let pool = get_pool().await;
        let set_cmd = Cmd::set("hi", Hello { name: String::from("myname") });
        let _: () = pool.query(set_cmd).await.unwrap();

        let get_cmd = Cmd::get("hi");
        let hi = pool.query::<Hello>(get_cmd).await.unwrap();
    
        println!("{:?}", hi)
    }

}