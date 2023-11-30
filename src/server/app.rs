use std::sync::{OnceLock, RwLock};

use config::Config;
use tracing::Level;
use tracing_subscriber::fmt::time::OffsetTime;

use super::router::router;

pub struct App {
    port: i32
}

impl App {
    pub fn new () -> Self {
        set_tracing();
        config();

        let conf_port = config().read().unwrap().get::<i32>("server.port");
        let port = conf_port.map_or(3000, |p| p);

        App{port}
    }

    #[tokio::main]
    pub async fn start(&self) {
        let listener =   tokio::net::TcpListener::bind(format!("0.0.0.0:{}", self.port)).await.unwrap();
        axum::serve(listener, router()).await.unwrap();
    }
}

fn set_tracing() {
    tracing_subscriber::fmt()
        .with_timer(OffsetTime::local_rfc_3339().expect("no local offset"))
        .with_line_number(true)
        .with_max_level(Level::INFO)
        // .json()
        .init();
}

fn config() -> &'static RwLock<Config> {
    static CONF: OnceLock<RwLock<Config>> = OnceLock::new();

    CONF.get_or_init(||
        RwLock::new(
            Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .expect("Read config Error")
        )
    )
}

// async fn graceful_shutdown() {
//     tokio::signal::ctrl_c()
//         .await.unwrap();

//     println!("88");
// }

#[cfg(test)]
mod test {
    use config::Config;


    #[test]
    fn config_test() {
        let p = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build().expect("no conf");

        println!("server port {}", p.get_string("server.port").expect("server.port error"));
    }

}